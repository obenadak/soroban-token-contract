// Bu dosya (test.rs), 'soroban-token-contract' kütüphanesinin birim testlerini içerir.
// '#[cfg(test)]' özniteliği, bu modülün sadece 'cargo test' komutu çalıştırıldığında derlenmesini sağlar.
// Testler, kontratın farklı fonksiyonlarının (mint, transfer, approve, burn, vb.)
// beklendiği gibi çalıştığını ve hata durumlarını doğru şekilde ele aldığını doğrular.
// Soroban SDK'sının test yardımcı araçları ('testutils') kullanılarak test ortamı oluşturulur,
// kontrat çağrıları simüle edilir ve yetkilendirme (auth) kontrolleri yapılır.

#![cfg(test)] // Bu modülün sadece testler sırasında derlenmesini sağlar.
extern crate std; // Testlerde standart kütüphanenin bazı özelliklerini (örneğin, std::vec!) kullanabilmek için
                  // 'std' kütüphanesini dışarıdan (extern) alır. Normalde '#![no_std]' ile derlenen kontratta
                  // bu olmaz, ancak test ortamında genellikle standart kütüphane kullanılabilir.

use crate::{contract::Token, TokenClient}; // Mevcut kütüphaneden (crate) 'Token' kontrat yapısını ve 'TokenClient' istemcisini içeri aktarır.
                                           // 'TokenClient', kontrat fonksiyonlarını testlerde daha kolay çağırmak için kullanılır.
use soroban_sdk::{
    symbol_short, // Kısa semboller ('Symbol' türü) oluşturmak için bir yardımcı makro. Genellikle fonksiyon adları için kullanılır.
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation}, // Soroban SDK'sının test yardımcı araçları:
                                                                        // 'Address as _': Adres oluşturma gibi testlere özel Address fonksiyonlarını getirir.
                                                                        // 'AuthorizedFunction': Bir kontrat fonksiyonunun yetkilendirilmiş çağrısını temsil eder.
                                                                        // 'AuthorizedInvocation': Yetkilendirilmiş bir çağrının tüm detaylarını (fonksiyon, alt çağrılar) tutar.
    Address, Env, IntoVal, Symbol, // Soroban SDK'sından temel türler:
                                   // 'Address': Adres türü.
                                   // 'Env': Test için sanal bir çalışma ortamı (environment).
                                   // 'IntoVal': Rust türlerini Soroban'ın 'Val' türüne dönüştürmek için bir trait.
                                   // 'Symbol': Sembol türü (kısa stringler).
};

// 'create_token' yardımcı fonksiyonu, testler için yeni bir token kontratı örneği oluşturur ve başlatır.
fn create_token<'a>(e: &Env, admin: &Address) -> TokenClient<'a> {
    // 'e': Test çalışma ortamı.
    // 'admin': Token kontratının yöneticisi olacak adres.
    // Fonksiyon, başlatılmış bir 'TokenClient' örneği döndürür.

    // Yeni bir 'TokenClient' oluşturur.
    // '&e.register_contract(None, Token {})' satırı, 'Token' kontratını test ortamına kaydeder
    // ve kontratın adresini döndürür. Bu adres, 'TokenClient'ı başlatmak için kullanılır.
    let token = TokenClient::new(e, &e.register_contract(None, Token {}));
    // Oluşturulan token kontratını 'initialize' fonksiyonu ile başlatır.
    // Parametreler: yönetici, ondalık sayısı (7), token adı ("name"), token sembolü ("symbol").
    // '".into_val(e)"' ile Rust string'leri Soroban 'Val' türüne dönüştürülür.
    token.initialize(admin, &7, &"name".into_val(e), &"symbol".into_val(e));
    token // Başlatılmış 'TokenClient'ı döndür.
}

#[test] // Bu fonksiyonun bir test senaryosu olduğunu belirtir.
fn test() { // Genel işlevselliği test eden bir test fonksiyonu.
    let e = Env::default(); // Varsayılan ayarlarla yeni bir test çalışma ortamı ('Env') oluşturur.
    e.mock_all_auths();     // Tüm yetkilendirme (auth) kontrollerini taklit eder (mock).
                            // Bu, her çağrının otomatik olarak yetkilendirilmiş gibi davranmasını sağlar
                            // ve 'require_auth()' çağrılarının başarılı olmasını garantiler.
                            // Hangi adresin hangi fonksiyonu çağırdığını 'e.auths()' ile kontrol edebiliriz.

    // Test için adresler oluşturur.
    let admin1 = Address::generate(&e); // Yönetici 1
    let admin2 = Address::generate(&e); // Yönetici 2
    let user1 = Address::generate(&e);  // Kullanıcı 1
    let user2 = Address::generate(&e);  // Kullanıcı 2
    let user3 = Address::generate(&e);  // Kullanıcı 3
    let token = create_token(&e, &admin1); // 'admin1' yöneticisiyle yeni bir token kontratı oluşturur.

    // 'user1' adresine 1000 token 'mint' (üret) eder.
    token.mint(&user1, &1000);
    // Yetkilendirme kontrolü: 'mint' fonksiyonunun 'admin1' tarafından çağrıldığını doğrular.
    assert_eq!(
        e.auths(), // Ortamdaki yetkilendirme kayıtlarını alır.
        std::vec![( // Beklenen yetkilendirme kaydı (bir vektör içinde).
            admin1.clone(), // Yetkilendiren adres.
            AuthorizedInvocation { // Yetkilendirilmiş çağrı detayları.
                function: AuthorizedFunction::Contract(( // Çağrılan fonksiyon bir kontrat fonksiyonu.
                    token.address.clone(),          // Kontrat adresi.
                    symbol_short!("mint"),          // Çağrılan fonksiyonun adı ("mint").
                    (&user1, 1000_i128).into_val(&e), // Fonksiyon argümanları.
                )),
                sub_invocations: std::vec![] // Alt çağrılar (bu durumda yok).
            }
        )]
    );
    assert_eq!(token.balance(&user1), 1000); // 'user1'in bakiyesinin 1000 olduğunu doğrular.

    // 'user2' adresine, 'user3' adına 500 token harcama izni ('approve') verir. İzin 200. deftere kadar geçerlidir.
    token.approve(&user2, &user3, &500, &200);
    // Yetkilendirme kontrolü: 'approve' fonksiyonunun 'user2' tarafından çağrıldığını doğrular.
    assert_eq!(
        e.auths(),
        std::vec![(
            user2.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    symbol_short!("approve"),
                    (&user2, &user3, 500_i128, 200_u32).into_val(&e),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );
    assert_eq!(token.allowance(&user2, &user3), 500); // 'user2'nin 'user3'e verdiği iznin 500 olduğunu doğrular.

    // 'user1'den 'user2'ye 600 token 'transfer' eder.
    token.transfer(&user1, &user2, &600);
    // Yetkilendirme kontrolü: 'transfer' fonksiyonunun 'user1' tarafından çağrıldığını doğrular.
    assert_eq!(
        e.auths(),
        std::vec![(
            user1.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    symbol_short!("transfer"),
                    (&user1, &user2, 600_i128).into_val(&e),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );
    assert_eq!(token.balance(&user1), 400); // 'user1'in kalan bakiyesinin 400 olduğunu doğrular.
    assert_eq!(token.balance(&user2), 600); // 'user2'nin yeni bakiyesinin 600 olduğunu doğrular.

    // 'user3' (spender), 'user2'den (from) 'user1'e (to) 400 token 'transfer_from' kullanarak transfer eder.
    token.transfer_from(&user3, &user2, &user1, &400);
    // Yetkilendirme kontrolü: 'transfer_from' fonksiyonunun 'user3' tarafından çağrıldığını doğrular.
    assert_eq!(
        e.auths(),
        std::vec![(
            user3.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    Symbol::new(&e, "transfer_from"), // Fonksiyon adı sembolü.
                    (&user3, &user2, &user1, 400_i128).into_val(&e),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );
    assert_eq!(token.balance(&user1), 800); // 'user1'in yeni bakiyesini doğrular (400 + 400).
    assert_eq!(token.balance(&user2), 200); // 'user2'nin kalan bakiyesini doğrular (600 - 400).

    // 'user1'den 'user3'e 300 token transfer eder.
    token.transfer(&user1, &user3, &300);
    assert_eq!(token.balance(&user1), 500); // 'user1'in kalan bakiyesini doğrular (800 - 300).
    assert_eq!(token.balance(&user3), 300); // 'user3'ün yeni bakiyesini doğrular.

    // Kontratın yöneticisini 'admin1'den 'admin2'ye değiştirir ('set_admin').
    token.set_admin(&admin2);
    // Yetkilendirme kontrolü: 'set_admin' fonksiyonunun 'admin1' (eski yönetici) tarafından çağrıldığını doğrular.
    assert_eq!(
        e.auths(),
        std::vec![(
            admin1.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    symbol_short!("set_admin"),
                    (&admin2,).into_val(&e), // Argüman bir tuple içinde.
                )),
                sub_invocations: std::vec![]
            }
        )]
    );

    // İzin miktarını önce 500'e artırır, sonra 0'a düşürür.
    token.approve(&user2, &user3, &500, &200); // İzni 500'e ayarla.
    assert_eq!(token.allowance(&user2, &user3), 500); // İznin 500 olduğunu doğrula.
    token.approve(&user2, &user3, &0, &200);   // İzni 0'a ayarla.
    // Yetkilendirme kontrolü: İkinci 'approve' çağrısının 'user2' tarafından yapıldığını doğrular.
    assert_eq!(
        e.auths(),
        std::vec![(
            user2.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    symbol_short!("approve"),
                    (&user2, &user3, 0_i128, 200_u32).into_val(&e),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );
    assert_eq!(token.allowance(&user2, &user3), 0); // İznin 0 olduğunu doğrular.
}

#[test] // Token yakma (burn) işlevselliğini test eder.
fn test_burn() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let user1 = Address::generate(&e);
    let user2 = Address::generate(&e); // Spender (harcayıcı) rolünde olacak.
    let token = create_token(&e, &admin);

    token.mint(&user1, &1000); // 'user1'e 1000 token üret.
    assert_eq!(token.balance(&user1), 1000); // Bakiyeyi doğrula.

    // 'user1', 'user2'ye 500 token yakma/harcama izni verir.
    token.approve(&user1, &user2, &500, &200);
    assert_eq!(token.allowance(&user1, &user2), 500); // İzni doğrula.

    // 'user2' (spender), 'user1'in (from) hesabından 500 token yakar ('burn_from').
    token.burn_from(&user2, &user1, &500);
    // Yetkilendirme kontrolü: 'burn_from' fonksiyonunun 'user2' tarafından çağrıldığını doğrular.
    assert_eq!(
        e.auths(),
        std::vec![(
            user2.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    symbol_short!("burn_from"),
                    (&user2, &user1, 500_i128).into_val(&e),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );

    assert_eq!(token.allowance(&user1, &user2), 0); // Harcama izninin 0'a düştüğünü doğrular.
    assert_eq!(token.balance(&user1), 500);      // 'user1'in bakiyesinin 500'e düştüğünü doğrular.
    assert_eq!(token.balance(&user2), 0);        // 'user2'nin bakiyesinin değişmediğini (0 olduğunu) doğrular.

    // 'user1' kendi hesabından kalan 500 token'ı yakar ('burn').
    token.burn(&user1, &500);
    // Yetkilendirme kontrolü: 'burn' fonksiyonunun 'user1' tarafından çağrıldığını doğrular.
    assert_eq!(
        e.auths(),
        std::vec![(
            user1.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    symbol_short!("burn"),
                    (&user1, 500_i128).into_val(&e),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );

    assert_eq!(token.balance(&user1), 0); // 'user1'in bakiyesinin 0 olduğunu doğrular.
    assert_eq!(token.balance(&user2), 0); // 'user2'nin bakiyesinin hala 0 olduğunu doğrular.
}

#[test] // Bu testin paniklemesi (hata vermesi) beklenir.
#[should_panic(expected = "insufficient balance")] // Beklenen panik mesajını belirtir.
fn transfer_insufficient_balance() { // Yetersiz bakiye durumunda transferin paniklemesini test eder.
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let user1 = Address::generate(&e);
    let user2 = Address::generate(&e);
    let token = create_token(&e, &admin);

    token.mint(&user1, &1000); // 'user1'e 1000 token üret.
    assert_eq!(token.balance(&user1), 1000);

    // 'user1'den 'user2'ye 1001 token (bakiyeden fazla) transfer etmeye çalışır.
    // Bu işlemin "insufficient balance" mesajıyla paniklemesi beklenir.
    token.transfer(&user1, &user2, &1001);
}

#[test]
#[should_panic(expected = "insufficient allowance")] // Beklenen panik mesajı.
fn transfer_from_insufficient_allowance() { // Yetersiz harcama izni durumunda 'transfer_from'un paniklemesini test eder.
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let user1 = Address::generate(&e); // Token sahibi (from)
    let user2 = Address::generate(&e); // Alıcı (to)
    let user3 = Address::generate(&e); // Harcayıcı (spender)
    let token = create_token(&e, &admin);

    token.mint(&user1, &1000); // 'user1'e 1000 token üret.
    assert_eq!(token.balance(&user1), 1000);

    // 'user1', 'user3'e 100 token harcama izni verir.
    token.approve(&user1, &user3, &100, &200);
    assert_eq!(token.allowance(&user1, &user3), 100);

    // 'user3', 'user1'den 'user2'ye 101 token (izinden fazla) transfer etmeye çalışır.
    // Bu işlemin "insufficient allowance" mesajıyla paniklemesi beklenir.
    token.transfer_from(&user3, &user1, &user2, &101);
}

#[test]
#[should_panic(expected = "already initialized")] // Beklenen panik mesajı.
fn initialize_already_initialized() { // Kontrat zaten başlatılmışken tekrar 'initialize' çağrılmasının paniklemesini test eder.
    let e = Env::default();
    let admin = Address::generate(&e);
    let token = create_token(&e, &admin); // Kontrat oluşturulur ve 'create_token' içinde zaten başlatılır.

    // Zaten başlatılmış olan kontratı tekrar başlatmaya çalışır.
    // Bu işlemin "already initialized" mesajıyla paniklemesi beklenir.
    token.initialize(&admin, &10, &"name".into_val(&e), &"symbol".into_val(&e));
}

#[test]
#[should_panic(expected = "Decimal must fit in a u8")] // Beklenen panik mesajı.
fn decimal_is_over_max() { // Ondalık sayısının u8 sınırını aşması durumunda 'initialize'ın paniklemesini test eder.
    let e = Env::default();
    let admin = Address::generate(&e);
    // TokenClient'ı doğrudan oluşturur, 'create_token' gibi otomatik başlatma yapmaz.
    let token = TokenClient::new(&e, &e.register_contract(None, Token {}));
    // 'initialize' fonksiyonunu, u8'in maksimum değerinden (255) büyük bir ondalık sayısıyla çağırır.
    // u32::from(u8::MAX) + 1, 256 değerini verir.
    // Bu işlemin "Decimal must fit in a u8" mesajıyla paniklemesi beklenir.
    token.initialize(
        &admin,
        &(u32::from(u8::MAX) + 1), // Ondalık sayısı (256)
        &"name".into_val(&e),
        &"symbol".into_val(&e),
    );
}