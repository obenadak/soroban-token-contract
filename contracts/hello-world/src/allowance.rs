// Bu dosya (allowance.rs), 'soroban-token-contract' kütüphanesinin 'allowance' modülünü oluşturur.
// Bu modül, token sahiplerinin (from) başka bir adrese (spender), kendi tokenlarından belirli bir miktarını
// harcama izni vermesini sağlayan "allowance" (izin) mekanizmasını yönetir.
// ERC-20 standardındaki `approve`, `allowance` ve `transferFrom` (dolaylı olarak `spend_allowance` ile)
// fonksiyonlarına benzer işlevsellik sağlar.
// İzinler, belirli bir miktarla ve bir son geçerlilik defteri (expiration_ledger) ile tanımlanır.

use crate::storage_types::{AllowanceDataKey, AllowanceValue, DataKey}; // Mevcut kütüphanenin 'storage_types' modülünden
                                                                      // 'AllowanceDataKey', 'AllowanceValue' ve 'DataKey'
                                                                      // türlerini içeri aktarır.
                                                                      // 'AllowanceDataKey': Bir iznin sahibi (from) ve harcayıcısını (spender) tutan anahtar yapısı.
                                                                      // 'AllowanceValue': İzin verilen miktarı (amount) ve son geçerlilik defterini (expiration_ledger) tutan yapı.
                                                                      // 'DataKey': Genel depolama anahtarı enum'ı, burada 'Allowance' varyantı kullanılır.
use soroban_sdk::{Address, Env};                                     // soroban_sdk kütüphanesinden 'Address' ve 'Env' türlerini içeri aktarır.
                                                                      // 'Address': Bir hesabı veya kontratı temsil eder.
                                                                      // 'Env': Soroban çalışma zamanı ortamına erişim sağlar.

// 'read_allowance' fonksiyonu, belirli bir 'from' adresi tarafından 'spender' adresine verilen izni okur.
pub fn read_allowance(e: &Env, from: Address, spender: Address) -> AllowanceValue {
    // 'e': Soroban çalışma zamanı ortamı.
    // 'from': İzni veren adres.
    // 'spender': İzni kullanacak olan adres.
    // Fonksiyon, 'AllowanceValue' türünde bir izin değeri döndürür.

    // 'from' ve 'spender' adreslerini kullanarak bir 'AllowanceDataKey' oluştururuz.
    // Bu anahtar, 'DataKey::Allowance' enum varyantı içine sarılır ve depolamada izni bulmak için kullanılır.
    let key = DataKey::Allowance(AllowanceDataKey { from, spender });

    // Kontratın 'temporary' (geçici) depolama alanından belirtilen 'key' ile bir değer almaya çalışırız.
    // 'temporary' depolama, verilerin belirli bir süre sonra otomatik olarak silinebileceği bir alandır (TTL - Time To Live).
    // 'get::<_, AllowanceValue>()' ile dönen değerin 'AllowanceValue' tipinde olmasını bekleriz.
    if let Some(allowance) = e.storage().temporary().get::<_, AllowanceValue>(&key) {
        // Eğer 'key' ile eşleşen bir izin depolamada bulunursa ('Some(allowance)'):

        // Mevcut defter numarasını (ledger sequence) alırız. Bu, blokzincirdeki mevcut "zamanı" temsil eder.
        // Eğer iznin son geçerlilik defteri (expiration_ledger), mevcut defter numarasından küçükse,
        // izin süresi dolmuş demektir.
        if allowance.expiration_ledger < e.ledger().sequence() {
            // Süresi dolmuşsa, izin miktarı 0 olan yeni bir 'AllowanceValue' döndürürüz.
            // Son geçerlilik defteri orijinal değerini korur, bu bilgiye ihtiyaç duyulabilir.
            AllowanceValue {
                amount: 0, // İzin miktarı sıfırlanır.
                expiration_ledger: allowance.expiration_ledger, // Son geçerlilik defteri aynı kalır.
            }
        } else {
            // İzin süresi dolmamışsa, depolamadan okunan orijinal 'allowance' değerini döndürürüz.
            allowance
        }
    } else {
        // Eğer 'key' ile eşleşen bir izin depolamada bulunamazsa ('None'):
        // Bu, hiç izin verilmediği anlamına gelir. Bu durumda, miktarı 0 ve expiration_ledger'ı 0 olan
        // varsayılan bir 'AllowanceValue' döndürürüz.
        AllowanceValue {
            amount: 0,
            expiration_ledger: 0,
        }
    }
}

// 'write_allowance' fonksiyonu, belirli bir 'from' adresi tarafından 'spender' adresine
// yeni bir izin miktarı ve son geçerlilik defteri yazar veya mevcut olanı günceller.
pub fn write_allowance(
    e: &Env,               // Soroban çalışma zamanı ortamı.
    from: Address,         // İzni veren adres.
    spender: Address,      // İzni kullanacak olan adres.
    amount: i128,          // Verilecek izin miktarı.
    expiration_ledger: u32, // İznin son geçerli olacağı defter numarası.
) {
    // Verilen 'amount' ve 'expiration_ledger' ile yeni bir 'AllowanceValue' oluştururuz.
    let allowance = AllowanceValue {
        amount,
        expiration_ledger,
    };

    // Bir güvenlik kontrolü: Eğer izin miktarı 0'dan büyükse ve
    // belirlenen son geçerlilik defteri (expiration_ledger) mevcut defter numarasından küçükse,
    // bu mantıksız bir durumdur (geçmişte sona erecek bir izin verilemez).
    // Bu durumda program panikler (hata verir ve işlemi durdurur).
    if amount > 0 && expiration_ledger < e.ledger().sequence() {
        panic!("expiration_ledger is less than ledger seq when amount > 0")
    }

    // İzin için depolama anahtarını oluştururuz.
    let key = DataKey::Allowance(AllowanceDataKey { from, spender });
    // Oluşturulan 'allowance' değerini, 'key' kullanarak kontratın 'temporary' depolama alanına yazarız.
    // 'key.clone()' kullanılır çünkü 'set' metodu anahtarın sahipliğini alabilir veya anahtar birden fazla yerde kullanılabilir.
    e.storage().temporary().set(&key.clone(), &allowance);

    // Eğer verilen izin miktarı 0'dan büyükse (yani aktif bir izin veriliyorsa):
    if amount > 0 {
        // İznin ne kadar süreyle "canlı" kalacağını hesaplarız.
        // Bu, son geçerlilik defteri (expiration_ledger) ile mevcut defter numarası (e.ledger().sequence()) arasındaki farktır.
        // 'checked_sub' kullanılır çünkü 'expiration_ledger' mevcut defterden küçükse (yukarıdaki panikle zaten engellenmiş olmalı),
        // taşma olmadan güvenli bir çıkarma işlemi yapar ve 'None' döndürür. 'unwrap()' ile bu durumun
        // olmaması beklenir (panic ile kontrol edildiği için).
        let live_for = expiration_ledger
            .checked_sub(e.ledger().sequence())
            .unwrap(); // Eğer expiration_ledger < e.ledger().sequence() ise bu panic olur, ancak yukarıda kontrol edildi.

        // Depolamadaki bu 'key' için Yaşam Süresini (Time To Live - TTL) uzatırız.
        // 'extend_ttl' fonksiyonu, bir anahtarın ne kadar süreyle daha geçici depolamada kalacağını ayarlar.
        // İlk 'live_for' parametresi, en az ne kadar süre daha kalması gerektiğini (lower bound),
        // ikinci 'live_for' parametresi ise en fazla ne kadar süre kalabileceğini (upper bound) belirtir.
        // Burada ikisi de aynı değere ayarlanarak tam olarak 'live_for' defter sayısı kadar daha yaşaması sağlanır.
        e.storage().temporary().extend_ttl(&key, live_for, live_for)
    }
}

// 'spend_allowance' fonksiyonu, 'spender' adresinin 'from' adresi adına verilen izinden
// belirli bir 'amount' (miktar) harcamasını sağlar.
pub fn spend_allowance(e: &Env, from: Address, spender: Address, amount: i128) {
    // 'e': Soroban çalışma zamanı ortamı.
    // 'from': Token sahibi, izni veren adres.
    // 'spender': İzni kullanarak token harcayacak olan adres.
    // 'amount': Harcanmak istenen token miktarı.

    // Öncelikle, 'from' tarafından 'spender'a verilen mevcut izni okuruz.
    // 'from.clone()' ve 'spender.clone()' kullanılır çünkü 'read_allowance' adreslerin sahipliğini almaz
    // ama klonlama burada açıkça yapılarak sahiplik kurallarına uyum sağlanır.
    let allowance = read_allowance(e, from.clone(), spender.clone());

    // Eğer mevcut izin miktarı (allowance.amount), harcanmak istenen miktardan (amount) az ise,
    // yeterli izin yok demektir. Bu durumda program panikler.
    if allowance.amount < amount {
        panic!("insufficient allowance");
    }

    // Yeterli izin varsa, izni güncelleriz.
    // Yeni izin miktarı, eski izin miktarından harcanan miktar çıkarılarak bulunur.
    // Son geçerlilik defteri (expiration_ledger) değişmez.
    write_allowance(
        e,
        from,                             // İzni veren adres.
        spender,                          // İzni kullanan adres.
        allowance.amount - amount,        // Kalan izin miktarı.
        allowance.expiration_ledger,      // Son geçerlilik defteri aynı kalır.
    );
}