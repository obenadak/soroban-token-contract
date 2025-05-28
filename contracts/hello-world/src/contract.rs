// Bu dosya (contract.rs), 'soroban-token-contract' kütüphanesinin ana akıllı kontrat mantığını içerir.
// 'Token' adında bir kontrat tanımlar ve bu kontrat için hem özel işlevler (initialize, mint, set_admin, freeze_account, unfreeze_account)
// hem de Soroban'ın standart token arayüzünü (soroban_sdk::token::Interface) uygular.
// Bu sayede, kontrat hem temel token işlemlerini (transfer, bakiye sorgulama, onaylama vb.)
// hem de kontrata özgü yönetim ve ek özellikleri (hesap dondurma gibi) destekler.
// Diğer modüllerde (admin, allowance, balance, metadata) tanımlanan fonksiyonları kullanarak
// kontratın durumunu yönetir ve işlemler gerçekleştirir.

use crate::admin::{has_administrator, read_administrator, write_administrator}; // Yönetici (admin) ile ilgili fonksiyonları 'admin' modülünden alır.
use crate::allowance::{read_allowance, spend_allowance, write_allowance};     // Harcama izinleri (allowance) ile ilgili fonksiyonları 'allowance' modülünden alır.
use crate::balance::{read_balance, receive_balance, spend_balance};           // Bakiye (balance) ile ilgili fonksiyonları 'balance' modülünden alır.
use crate::metadata::{read_decimal, read_name, read_symbol, write_metadata};   // Token meta verileri (isim, sembol, ondalık) ile ilgili fonksiyonları 'metadata' modülünden alır.
use crate::storage_types::{INSTANCE_BUMP_AMOUNT, INSTANCE_LIFETIME_THRESHOLD}; // Kontrat örneği depolaması için TTL (Time-To-Live) sabitlerini 'storage_types' modülünden alır.
use crate::storage_types::DataKey;                                            // Genel depolama anahtarı enum'ını 'storage_types' modülünden alır.
use soroban_sdk::token::{self, Interface as _};                              // Soroban SDK'sının standart token arayüzünü ve ilgili özellikleri alır.
                                                                              // 'Interface as _' yapısı, arayüzdeki metodları doğrudan çağırabilmek için kullanılır.
use soroban_sdk::{contract, contractimpl, Address, Env, String};              // Soroban SDK'sının temel kontrat geliştirme araçlarını alır:
                                                                              // 'contract': Bir struct'ı Soroban kontratı olarak işaretler.
                                                                              // 'contractimpl': Bir impl bloğunu kontrat fonksiyonlarını içeriyor olarak işaretler.
                                                                              // 'Address': Soroban adres türü.
                                                                              // 'Env': Kontratın çalıştığı ortam (environment).
                                                                              // 'String': Soroban için optimize edilmiş string türü.
use soroban_token_sdk::metadata::TokenMetadata;                               // soroban_token_sdk'dan 'TokenMetadata' yapısını alır, token meta verilerini tutmak için kullanılır.
use soroban_token_sdk::TokenUtils;                                            // soroban_token_sdk'dan 'TokenUtils' alır, genellikle standart token olaylarını (events) yayınlamak için kullanılır.


// 'check_nonnegative_amount' yardımcı fonksiyonu, verilen miktarın negatif olup olmadığını kontrol eder.
// Negatif miktarlara izin verilmez ve bu durumda program panikler.
fn check_nonnegative_amount(amount: i128) {
    if amount < 0 { // Eğer miktar 0'dan küçükse...
        panic!("negative amount is not allowed: {}", amount) // Hata mesajıyla programı durdur.
    }
}

// 'is_account_frozen' yardımcı fonksiyonu, bir hesabın dondurulup dondurulmadığını kontrol eder.
fn is_account_frozen(e: &Env, account: &Address) -> bool { // 'e': Çalışma ortamı, 'account': Kontrol edilecek hesap adresi.
    let key = DataKey::Frozen(account.clone());            // Hesap için dondurma durumunu saklayan depolama anahtarını oluştur.
                                                           // '.clone()' ile 'account' adresinin bir kopyası kullanılır.
    e.storage()                                            // Kontratın depolama alanına eriş.
        .instance()                                        // 'instance' (örnek) depolama türünü kullan.
        .get::<_, bool>(&key)                              // Belirtilen anahtarla depodan boolean bir değer oku.
        .unwrap_or(false)                                  // Eğer değer varsa onu, yoksa 'false' (dondurulmamış) döndür.
}

// 'emit_custom_event' yardımcı fonksiyonu, özel olayları (events) yayınlamak için kullanılır.
fn emit_custom_event(e: &Env, event_type: &str, admin: Address, account: Address) { // 'e': Çalışma ortamı, 'event_type': Olayın türünü belirten string, 'admin': Olayla ilişkili admin, 'account': Olayla ilişkili hesap.
    e.events()                                             // Ortamın olay yöneticisine eriş.
        .publish((event_type, admin, account), ());        // Belirtilen verilerle bir olay yayınla. İkinci tuple `()` boş veri yükünü temsil eder.
}

#[contract] // Bu struct'ın bir Soroban akıllı kontratı olduğunu belirtir.
pub struct Token; // 'Token' adlı kontrat yapısı. Bu yapı, kontratın durumunu değil, tipini tanımlar.
                  // Kontratın durumu 'Env' üzerinden erişilen depolamada tutulur.

#[contractimpl] // Bu blok, 'Token' kontratı için fonksiyonları (metodları) içerir.
impl Token {
    // 'initialize' fonksiyonu, kontratı ilk kez kurar. Sadece bir kez çağrılabilir.
    // Yöneticiyi (admin), ondalık sayısını, ismini ve sembolünü ayarlar.
    pub fn initialize(e: Env, admin: Address, decimal: u32, name: String, symbol: String) {
        // 'e': Çalışma ortamı.
        // 'admin': Kontratın yöneticisi olacak adres.
        // 'decimal': Token'ın ondalık basamak sayısı.
        // 'name': Token'ın adı.
        // 'symbol': Token'ın sembolü.

        if has_administrator(&e) { // Eğer 'admin' modülündeki 'has_administrator' fonksiyonu true döndürürse (yani yönetici zaten ayarlanmışsa)...
            panic!("already initialized") // Kontrat zaten başlatılmışsa hata ver.
        }
        write_administrator(&e, &admin); // 'admin' modülünü kullanarak yönetici adresini depolamaya yaz.

        if decimal > u8::MAX.into() { // Eğer ondalık sayısı bir u8'e sığmayacak kadar büyükse...
                                      // u8::MAX, 8-bitlik işaretsiz bir tamsayının alabileceği maksimum değerdir (255).
                                      // '.into()' ile u32'den u8 karşılaştırması için dönüştürme yapılır.
            panic!("Decimal must fit in a u8"); // Hata ver. Standart token arayüzü ondalık için u8 bekler.
        }

        // 'metadata' modülünü kullanarak token'ın meta verilerini (ondalık, isim, sembol) depolamaya yaz.
        write_metadata(
            &e,
            TokenMetadata { // 'TokenMetadata' yapısını oluştur.
                decimal,
                name,
                symbol,
            },
        )
    }

    // 'mint' fonksiyonu, belirli bir 'to' adresine 'amount' kadar yeni token üretir (basar).
    // Sadece yönetici (admin) tarafından çağrılabilir.
    pub fn mint(e: Env, to: Address, amount: i128) {
        // 'e': Çalışma ortamı.
        // 'to': Token'ların gönderileceği adres.
        // 'amount': Üretilecek token miktarı.

        check_nonnegative_amount(amount); // Miktarın negatif olmadığını kontrol et.
        let admin = read_administrator(&e); // Mevcut yönetici adresini oku.
        admin.require_auth(); // Bu fonksiyonun çağrılabilmesi için yönetici adresinin işlemi imzalamış olmasını zorunlu kıl.

        // Kontrat örneğinin depolamadaki Yaşam Süresini (TTL) uzat.
        // Bu, kontratın depolama ücretleri nedeniyle silinmesini engeller.
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        receive_balance(&e, to.clone(), amount); // 'balance' modülünü kullanarak 'to' adresinin bakiyesine 'amount' ekle.
                                                 // 'to.clone()' ile adresin kopyası kullanılır.
        TokenUtils::new(&e).events().mint(admin, to, amount); // Standart 'mint' olayını yayınla.
    }

    // 'set_admin' fonksiyonu, kontratın yöneticisini 'new_admin' olarak değiştirir.
    // Sadece mevcut yönetici tarafından çağrılabilir.
    pub fn set_admin(e: Env, new_admin: Address) {
        // 'e': Çalışma ortamı.
        // 'new_admin': Yeni yönetici olacak adres.

        let admin = read_administrator(&e); // Mevcut yönetici adresini oku.
        admin.require_auth(); // Mevcut yöneticinin işlemi imzalamasını zorunlu kıl.

        // Kontrat örneğinin TTL'sini uzat.
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        write_administrator(&e, &new_admin); // 'admin' modülünü kullanarak yeni yönetici adresini depolamaya yaz.
        TokenUtils::new(&e).events().set_admin(admin, new_admin); // Standart 'set_admin' olayını yayınla (eski ve yeni admini bildirir).
    }

    // 'freeze_account' fonksiyonu, belirtilen 'account' adresini dondurur.
    // Dondurulmuş hesaplar token transfer edemez veya yakamaz.
    // Sadece yönetici tarafından çağrılabilir.
    pub fn freeze_account(e: Env, account: Address) {
        // 'e': Çalışma ortamı.
        // 'account': Dondurulacak hesap adresi.

        let admin = read_administrator(&e); // Mevcut yönetici adresini oku.
        admin.require_auth(); // Yöneticinin işlemi imzalamasını zorunlu kıl.

        // Kontrat örneğinin TTL'sini uzat.
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        // Hesabı dondurulmuş olarak işaretlemek için depolamaya yazarız.
        let key = DataKey::Frozen(account.clone()); // Dondurma durumu için depolama anahtarı.
        e.storage().instance().set(&key, &true);    // Anahtarın değerini 'true' (dondurulmuş) olarak ayarla.

       // Özel bir 'freeze_account' olayı yayınla.
       emit_custom_event(&e, "freeze_account", admin, account);
    }

    // 'unfreeze_account' fonksiyonu, belirtilen 'account' adresinin dondurulmasını kaldırır.
    // Sadece yönetici tarafından çağrılabilir.
    pub fn unfreeze_account(e: Env, account: Address) {
        // 'e': Çalışma ortamı.
        // 'account': Dondurulması kaldırılacak hesap adresi.

        let admin = read_administrator(&e); // Mevcut yönetici adresini oku.
        admin.require_auth(); // Yöneticinin işlemi imzalamasını zorunlu kıl.

        // Kontrat örneğinin TTL'sini uzat.
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        // Dondurulmuş durumunu depolamadan kaldırırız.
        let key = DataKey::Frozen(account.clone()); // Dondurma durumu için depolama anahtarı.
        e.storage().instance().remove(&key);        // Anahtarı ve değerini depodan sil.

        // Özel bir 'unfreeze_account' olayı yayınla.
        emit_custom_event(&e, "unfreeze_account", admin, account);
    }
}

#[contractimpl] // Bu blok, 'Token' kontratı için standart 'soroban_sdk::token::Interface' arayüzünü uygular.
impl token::Interface for Token {
    // 'allowance' fonksiyonu, 'from' adresinin 'spender' adresine ne kadar token harcama izni verdiğini döndürür.
    fn allowance(e: Env, from: Address, spender: Address) -> i128 {
        // Kontrat örneğinin TTL'sini uzat.
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        // 'allowance' modülündeki 'read_allowance' fonksiyonunu çağırarak izin miktarını oku ve döndür.
        read_allowance(&e, from, spender).amount
    }

    // 'approve' fonksiyonu, 'from' adresinin 'spender' adresine 'amount' kadar token harcama izni vermesini sağlar.
    // 'expiration_ledger', bu iznin ne zaman sona ereceğini belirtir.
    fn approve(e: Env, from: Address, spender: Address, amount: i128, expiration_ledger: u32) {
        from.require_auth(); // 'from' adresinin (izin veren) işlemi imzalamasını zorunlu kıl.

        check_nonnegative_amount(amount); // Miktarın negatif olmadığını kontrol et.

        // Kontrat örneğinin TTL'sini uzat.
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        // 'allowance' modülündeki 'write_allowance' fonksiyonunu çağırarak izni depolamaya yaz.
        write_allowance(&e, from.clone(), spender.clone(), amount, expiration_ledger);
        // Standart 'approve' olayını yayınla.
        TokenUtils::new(&e)
            .events()
            .approve(from, spender, amount, expiration_ledger);
    }

    // 'balance' fonksiyonu, 'id' adresinin token bakiyesini döndürür.
    fn balance(e: Env, id: Address) -> i128 {
        // Kontrat örneğinin TTL'sini uzat.
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        // 'balance' modülündeki 'read_balance' fonksiyonunu çağırarak bakiyeyi oku ve döndür.
        read_balance(&e, id)
    }

    // 'transfer' fonksiyonu, 'from' adresinden 'to' adresine 'amount' kadar token transfer eder.
    fn transfer(e: Env, from: Address, to: Address, amount: i128) {
        from.require_auth(); // 'from' adresinin (gönderici) işlemi imzalamasını zorunlu kıl.

        check_nonnegative_amount(amount); // Miktarın negatif olmadığını kontrol et.

        // Kontrat örneğinin TTL'sini uzat.
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        // Göndericinin ('from') hesabı dondurulmuş mu kontrol et.
        if is_account_frozen(&e, &from) {
            panic!("Hesap dondurulmuş ve token transfer edilemez"); // Dondurulmuşsa hata ver.
        }

        // Transferi gerçekleştir:
        spend_balance(&e, from.clone(), amount);    // 'from' adresinin bakiyesini azalt.
        receive_balance(&e, to.clone(), amount);    // 'to' adresinin bakiyesini artır.
        TokenUtils::new(&e).events().transfer(from, to, amount); // Standart 'transfer' olayını yayınla.
    }

    // 'transfer_from' fonksiyonu, 'spender' adresinin 'from' adresinden aldığı izni kullanarak
    // 'from' adresinden 'to' adresine 'amount' kadar token transfer eder.
    fn transfer_from(e: Env, spender: Address, from: Address, to: Address, amount: i128) {
        spender.require_auth(); // 'spender' adresinin (harcama iznini kullanan) işlemi imzalamasını zorunlu kıl.

        check_nonnegative_amount(amount); // Miktarın negatif olmadığını kontrol et.

        // Kontrat örneğinin TTL'sini uzat.
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        // Göndericinin ('from') hesabı dondurulmuş mu kontrol et.
        if is_account_frozen(&e, &from) {
            panic!("Hesap dondurulmuş ve token transfer edilemez"); // Dondurulmuşsa hata ver.
        }

        // Transferi gerçekleştir:
        spend_allowance(&e, from.clone(), spender, amount); // 'spender'ın 'from' adına olan harcama iznini azalt.
        spend_balance(&e, from.clone(), amount);            // 'from' adresinin bakiyesini azalt.
        receive_balance(&e, to.clone(), amount);            // 'to' adresinin bakiyesini artır.
        TokenUtils::new(&e).events().transfer(from, to, amount); // Standart 'transfer' olayını yayınla.
    }

    // 'burn' fonksiyonu, 'from' adresinin kendi bakiyesinden 'amount' kadar token yakar (yok eder).
    fn burn(e: Env, from: Address, amount: i128) {
        from.require_auth(); // 'from' adresinin (token yakan) işlemi imzalamasını zorunlu kıl.

        check_nonnegative_amount(amount); // Miktarın negatif olmadığını kontrol et.

        // Kontrat örneğinin TTL'sini uzat.
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        // 'from' adresinin hesabı dondurulmuş mu kontrol et.
        if is_account_frozen(&e, &from) {
            panic!("Hesap dondurulmuş ve token yakılamaz"); // Dondurulmuşsa hata ver.
        }

        // Yakma işlemini gerçekleştir:
        spend_balance(&e, from.clone(), amount); // 'from' adresinin bakiyesini azalt.
        TokenUtils::new(&e).events().burn(from, amount); // Standart 'burn' olayını yayınla.
    }

    // 'burn_from' fonksiyonu, 'spender' adresinin 'from' adresinden aldığı izni kullanarak
    // 'from' adresinin bakiyesinden 'amount' kadar token yakar.
    fn burn_from(e: Env, spender: Address, from: Address, amount: i128) {
        spender.require_auth(); // 'spender' adresinin işlemi imzalamasını zorunlu kıl.

        check_nonnegative_amount(amount); // Miktarın negatif olmadığını kontrol et.

        // Kontrat örneğinin TTL'sini uzat.
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

         // 'from' adresinin hesabı dondurulmuş mu kontrol et.
         if is_account_frozen(&e, &from) {
            panic!("Hesap dondurulmuş ve token yakılamaz"); // Dondurulmuşsa hata ver.
        }

        // Yakma işlemini gerçekleştir:
        spend_allowance(&e, from.clone(), spender, amount); // 'spender'ın 'from' adına olan harcama iznini azalt.
        spend_balance(&e, from.clone(), amount);            // 'from' adresinin bakiyesini azalt.
        TokenUtils::new(&e).events().burn(from, amount);    // Standart 'burn' olayını yayınla.
    }

    // 'decimals' fonksiyonu, token'ın ondalık basamak sayısını döndürür.
    fn decimals(e: Env) -> u32 {
        read_decimal(&e) // 'metadata' modülünden ondalık sayısını oku ve döndür.
    }

    // 'name' fonksiyonu, token'ın adını döndürür.
    fn name(e: Env) -> String {
        read_name(&e) // 'metadata' modülünden adı oku ve döndür.
    }

    // 'symbol' fonksiyonu, token'ın sembolünü döndürür.
    fn symbol(e: Env) -> String {
        read_symbol(&e) // 'metadata' modülünden sembolü oku ve döndür.
    }
}