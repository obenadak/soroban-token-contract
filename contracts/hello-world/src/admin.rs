// Bu dosya (admin.rs), 'soroban-token-contract' kütüphanesinin 'admin' modülünü oluşturur.
// Bu modülün temel amacı, akıllı kontratın yönetici (administrator) adresini yönetmektir.
// Yönetici, genellikle kontrat üzerinde özel yetkilere sahip olan bir adrestir (örneğin, kontratı yükseltme,
// belirli parametreleri değiştirme gibi). Bu modül, yöneticinin varlığını kontrol etmek,
// yönetici adresini okumak ve yeni bir yönetici adresi yazmak için fonksiyonlar içerir.

use soroban_sdk::{Address, Env}; // soroban_sdk kütüphanesinden 'Address' ve 'Env' türlerini içeri aktarır.
                                 // 'Address': Soroban'daki bir hesabı veya kontratı temsil eden adres türü.
                                 // 'Env': Mevcut Soroban çalışma zamanı ortamına erişim sağlayan yapı (environment).
                                 //          Kontrat depolamasına, güncel defter bilgilerine vb. erişmek için kullanılır.

use crate::storage_types::DataKey; // Mevcut kütüphanenin (crate) 'storage_types' modülünden 'DataKey' enum'ını içeri aktarır.
                                   // 'DataKey', kontrat depolamasında verileri organize etmek için kullanılan anahtarları tanımlar.

// 'has_administrator' fonksiyonu, kontrat depolamasında bir yönetici adresinin kayıtlı olup olmadığını kontrol eder.
pub fn has_administrator(e: &Env) -> bool { // 'e' parametresi, Soroban çalışma zamanı ortamına bir referanstır.
                                            // Fonksiyon, bir boolean (true/false) değer döndürür.
    let key = DataKey::Admin;               // Depolama anahtarı olarak 'DataKey' enum'ının 'Admin' varyantını kullanır.
                                            // Bu, yönetici adresinin depolamada bu anahtar altında saklandığını gösterir.
    e.storage()                             // 'Env' üzerinden kontratın depolama alanına erişir.
        .instance()                         // 'instance' depolama alanını seçer. Bu, kontratın ömrü boyunca devam eden veriler için kullanılır.
        .has(&key)                          // Belirtilen 'key' ile depolamada bir değer olup olmadığını kontrol eder.
                                            // Varsa 'true', yoksa 'false' döndürür.
}

// 'read_administrator' fonksiyonu, kontrat depolamasından kayıtlı yönetici adresini okur.
pub fn read_administrator(e: &Env) -> Address { // 'e' parametresi, Soroban çalışma zamanı ortamına bir referanstır.
                                               // Fonksiyon, bir 'Address' türünde yönetici adresi döndürür.
    let key = DataKey::Admin;                  // Yönetici adresi için depolama anahtarını belirler.
    e.storage()                                // Kontratın depolama alanına erişir.
        .instance()                            // 'instance' depolama alanını kullanır.
        .get(&key)                             // Belirtilen 'key' ile depolamadan değeri alır.
                                               // Bu, bir `Option<Address>` döndürür.
        .unwrap()                              // `Option` içerisindeki değeri çıkarır. Eğer değer `None` ise (yani anahtar bulunamazsa)
                                               // program panikler. Bu kullanım, yönetici adresinin her zaman var olması
                                               // gerektiği varsayımına dayanır (muhtemelen kontrat başlatıldığında ayarlanmıştır).
                                               // Daha güvenli bir yaklaşım, `expect()` veya `match` ile `None` durumunu ele almak olabilir.
}

// 'write_administrator' fonksiyonu, kontrat depolamasına yeni bir yönetici adresi yazar (veya mevcut olanı günceller).
pub fn write_administrator(e: &Env, id: &Address) { // 'e': Soroban çalışma zamanı ortamı.
                                                    // 'id': Yeni yönetici olarak ayarlanacak 'Address' referansı.
    let key = DataKey::Admin;                       // Yönetici adresi için depolama anahtarını belirler.
    e.storage()                                     // Kontratın depolama alanına erişir.
        .instance()                                 // 'instance' depolama alanını kullanır.
        .set(&key, id);                             // Belirtilen 'key' altına verilen 'id' (yönetici adresi) değerini yazar/günceller.
}