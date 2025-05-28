// Bu dosya (balance.rs), 'soroban-token-contract' kütüphanesinin 'balance' modülünü oluşturur.
// Bu modülün temel amacı, token sahiplerinin (adreslerin) token bakiyelerini yönetmektir.
// Fonksiyonlar, bir adresin bakiyesini okumak, bir adrese token yatırmak (bakiye artırmak)
// ve bir adresten token harcamak (bakiye azaltmak) için kullanılır.
// Bakiyeler, Soroban'ın 'persistent' (kalıcı) depolama alanında saklanır ve
// depolama ücretlerini yönetmek için TTL (Time-To-Live) mekanizmalarıyla güncellenir.

use crate::storage_types::{DataKey, BALANCE_BUMP_AMOUNT, BALANCE_LIFETIME_THRESHOLD};
// Mevcut kütüphanenin (crate) 'storage_types' modülünden belirli öğeleri içeri aktarır:
// - 'DataKey': Depolama anahtarlarını tanımlayan enum. Burada 'Balance' varyantı kullanılır.
// - 'BALANCE_BUMP_AMOUNT': Kalıcı depolamadaki bir girdinin TTL'sinin ne kadar artırılacağını belirten sabit.
// - 'BALANCE_LIFETIME_THRESHOLD': Bir girdinin TTL'sinin artırılması gerekip gerekmediğini belirlemek için kullanılan eşik değer.
use soroban_sdk::{Address, Env}; // soroban_sdk kütüphanesinden 'Address' ve 'Env' türlerini içeri aktarır.
                                 // 'Address': Bir hesabı veya kontratı temsil eder.
                                 // 'Env': Soroban çalışma zamanı ortamına erişim sağlar.

// 'read_balance' fonksiyonu, belirtilen 'addr' adresinin token bakiyesini okur.
pub fn read_balance(e: &Env, addr: Address) -> i128 { // 'e': Soroban çalışma zamanı ortamı.
                                                      // 'addr': Bakiyesi okunacak adres.
                                                      // Fonksiyon, 'i128' türünde bir bakiye değeri döndürür.
    let key = DataKey::Balance(addr);                 // Adresi kullanarak bakiye için bir 'DataKey' oluşturur.
                                                      // Bu anahtar, depolamada bakiyeyi bulmak için kullanılır.

    // Kontratın 'persistent' (kalıcı) depolama alanından belirtilen 'key' ile bir değer (bakiye) almaya çalışırız.
    // 'get::<DataKey, i128>(&key)' ile hem anahtarın hem de değerin tipini belirtiriz.
    if let Some(balance) = e.storage().persistent().get::<DataKey, i128>(&key) {
        // Eğer 'key' ile eşleşen bir bakiye depolamada bulunursa ('Some(balance)'):

        // Bu bakiye girdisinin Yaşam Süresini (Time To Live - TTL) uzatırız.
        // Soroban'da kalıcı depolama ücretlendirilir ve verilerin "canlı" tutulması gerekir.
        // 'extend_ttl', girdinin TTL'sini belirli bir eşik (BALANCE_LIFETIME_THRESHOLD)
        // ve artış miktarı (BALANCE_BUMP_AMOUNT) kullanarak günceller.
        // Eğer mevcut TTL, 'BALANCE_LIFETIME_THRESHOLD' değerinden düşükse, TTL 'BALANCE_BUMP_AMOUNT' kadar artırılır.
        e.storage()
            .persistent()
            .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
        balance // Okunan bakiye değerini döndür.
    } else {
        // Eğer 'key' ile eşleşen bir bakiye depolamada bulunamazsa (yani adresin daha önce hiç bakiyesi olmadıysa):
        0 // Varsayılan olarak 0 bakiyesini döndür.
    }
}

// 'write_balance' fonksiyonu (bu modül içinde özeldir, 'pub' olmadığı için dışarıdan erişilemez),
// belirtilen 'addr' adresinin bakiyesini verilen 'amount' ile günceller veya yazar.
fn write_balance(e: &Env, addr: Address, amount: i128) { // 'e': Soroban çalışma zamanı ortamı.
                                                         // 'addr': Bakiyesi yazılacak adres.
                                                         // 'amount': Yazılacak yeni bakiye miktarı.
    let key = DataKey::Balance(addr);                    // Bakiye için depolama anahtarını oluşturur.
    e.storage().persistent().set(&key, &amount);         // Belirtilen 'key' altına yeni 'amount' (bakiye) değerini yazar.

    // Bakiye girdisinin TTL'sini güncelleriz, tıpkı 'read_balance' fonksiyonunda olduğu gibi.
    // Bu, verinin kalıcı depolamada aktif kalmasını sağlar.
    e.storage()
        .persistent()
        .extend_ttl(&key, BALANCE_LIFETIME_THRESHOLD, BALANCE_BUMP_AMOUNT);
}

// 'receive_balance' fonksiyonu, belirtilen 'addr' adresinin bakiyesine 'amount' kadar token ekler.
pub fn receive_balance(e: &Env, addr: Address, amount: i128) { // 'e': Soroban çalışma zamanı ortamı.
                                                               // 'addr': Token alacak adres.
                                                               // 'amount': Eklenecek token miktarı.
    let balance = read_balance(e, addr.clone());               // Önce adresin mevcut bakiyesini okuruz.
                                                               // 'addr.clone()' kullanılır çünkü 'read_balance' 'addr' alır
                                                               // ve 'addr' daha sonra 'write_balance' için tekrar kullanılır.
    write_balance(e, addr, balance + amount);                  // Mevcut bakiyeye 'amount' eklenir ve yeni bakiye depolamaya yazılır.
}

// 'spend_balance' fonksiyonu, belirtilen 'addr' adresinin bakiyesinden 'amount' kadar token harcar (azaltır).
pub fn spend_balance(e: &Env, addr: Address, amount: i128) { // 'e': Soroban çalışma zamanı ortamı.
                                                             // 'addr': Token harcayacak adres.
                                                             // 'amount': Harcanacak token miktarı.
    let balance = read_balance(e, addr.clone());             // Önce adresin mevcut bakiyesini okuruz.

    // Eğer mevcut bakiye (balance), harcanmak istenen miktardan (amount) az ise,
    // yeterli bakiye yok demektir. Bu durumda program panikler (hata verir ve işlemi durdurur).
    if balance < amount {
        panic!("insufficient balance");
    }
    write_balance(e, addr, balance - amount);                // Mevcut bakiyeden 'amount' çıkarılır ve yeni bakiye depolamaya yazılır.
}