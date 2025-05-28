// Bu dosya (metadata.rs), 'soroban-token-contract' kütüphanesinin 'metadata' modülünü oluşturur.
// Bu modül, token'ın meta verilerini (metadata) yönetmekten sorumludur.
// Meta veriler genellikle token'ın ondalık basamak sayısı (decimal), adı (name) ve sembolünü (symbol) içerir.
// Bu modül, bu bilgileri okumak ve yazmak için fonksiyonlar sağlar.
// İşlemler için 'soroban_token_sdk' kütüphanesindeki 'TokenUtils' ve 'TokenMetadata' yapılarını kullanır.
// 'TokenUtils' SDK'sı, meta verilerin standart bir şekilde depolanmasını ve erişilmesini kolaylaştırır.

use soroban_sdk::{Env, String}; // soroban_sdk kütüphanesinden 'Env' (çalışma ortamı) ve 'String' (Soroban string türü)
                                // türlerini içeri aktarır.
use soroban_token_sdk::{metadata::TokenMetadata, TokenUtils}; // soroban_token_sdk kütüphanesinden:
                                                              // - 'metadata::TokenMetadata': Token meta verilerini (decimal, name, symbol)
                                                              //   bir arada tutan yapıyı içeri aktarır.
                                                              // - 'TokenUtils': Token ile ilgili yardımcı fonksiyonlara, özellikle
                                                              //   meta veri yönetimine erişim sağlayan bir araç yapısını içeri aktarır.

// 'read_decimal' fonksiyonu, token'ın depolanmış ondalık basamak sayısını okur.
pub fn read_decimal(e: &Env) -> u32 { // 'e': Soroban çalışma zamanı ortamına bir referans.
                                     // Fonksiyon, bir 'u32' (ondalık sayısı) değeri döndürür.
    let util = TokenUtils::new(e);   // Verilen çalışma ortamı 'e' ile yeni bir 'TokenUtils' örneği oluşturur.
                                     // Bu 'util' nesnesi üzerinden meta veri fonksiyonlarına erişilir.
    util.metadata()                  // 'TokenUtils' üzerinden meta veri yönetimi kısmına erişir.
        .get_metadata()              // Depolamadan tüm 'TokenMetadata' yapısını okur.
        .decimal                     // Okunan 'TokenMetadata' yapısından 'decimal' alanını döndürür.
}

// 'read_name' fonksiyonu, token'ın depolanmış adını okur.
pub fn read_name(e: &Env) -> String { // 'e': Soroban çalışma zamanı ortamına bir referans.
                                    // Fonksiyon, bir 'String' (token adı) değeri döndürür.
    let util = TokenUtils::new(e);  // Yeni bir 'TokenUtils' örneği oluşturur.
    util.metadata()                 // Meta veri yönetimi kısmına erişir.
        .get_metadata()             // Depolamadan tüm 'TokenMetadata' yapısını okur.
        .name                       // Okunan 'TokenMetadata' yapısından 'name' alanını döndürür.
}

// 'read_symbol' fonksiyonu, token'ın depolanmış sembolünü okur.
pub fn read_symbol(e: &Env) -> String { // 'e': Soroban çalışma zamanı ortamına bir referans.
                                      // Fonksiyon, bir 'String' (token sembolü) değeri döndürür.
    let util = TokenUtils::new(e);    // Yeni bir 'TokenUtils' örneği oluşturur.
    util.metadata()                   // Meta veri yönetimi kısmına erişir.
        .get_metadata()               // Depolamadan tüm 'TokenMetadata' yapısını okur.
        .symbol                       // Okunan 'TokenMetadata' yapısından 'symbol' alanını döndürür.
}

// 'write_metadata' fonksiyonu, verilen 'TokenMetadata' yapısını depolamaya yazar.
// Bu, genellikle kontratın 'initialize' fonksiyonunda token'ın ilk meta verilerini ayarlamak için kullanılır.
pub fn write_metadata(e: &Env, metadata: TokenMetadata) { // 'e': Soroban çalışma zamanı ortamı.
                                                          // 'metadata': Yazılacak 'TokenMetadata' yapısı.
    let util = TokenUtils::new(e);                        // Yeni bir 'TokenUtils' örneği oluşturur.
    util.metadata()                                       // Meta veri yönetimi kısmına erişir.
        .set_metadata(&metadata);                         // Verilen 'metadata' referansını kullanarak depolamadaki meta verileri ayarlar/günceller.
}