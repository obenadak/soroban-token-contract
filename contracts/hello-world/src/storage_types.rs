// Bu dosya (storage_types.rs), 'soroban-token-contract' kütüphanesi için depolama ile ilgili
// çeşitli veri türlerini ve sabitleri tanımlar. Bu, kontratın durumunu (state) Soroban'ın
// depolama mekanizmalarında nasıl organize edeceğini ve yöneteceğini belirler.
// Özellikle, depolama anahtarları için kullanılan enum'ları, izin (allowance) verilerini tutan yapıları
// ve depolamadaki verilerin Yaşam Süresi (Time-To-Live - TTL) yönetimi için kullanılan sabitleri içerir.
// TTL sabitleri, depolama girişlerinin ne kadar süreyle "canlı" kalacağını ve ne zaman
// "bump" (uzatma/artırma) işlemi yapılması gerektiğini belirler.

use soroban_sdk::{contracttype, Address}; // soroban_sdk kütüphanesinden:
                                          // - 'contracttype': Bir Rust türünü (struct veya enum) Soroban kontratlarında
                                          //   depolanabilir ve kullanılabilir hale getiren bir makro (attribute).
                                          // - 'Address': Soroban'daki bir hesabı veya kontratı temsil eden adres türü.

// Sabitler, depolama girişlerinin Yaşam Süresi (TTL) yönetimi için kullanılır.
// Soroban'da depolama ücretlendirilir ve verilerin aktif tutulması için periyodik olarak TTL'lerinin
// "bump" edilmesi (artırılması) gerekir. Bu sabitler, bu bump işleminin ne zaman ve ne kadar
// yapılması gerektiğini tanımlar. Defter (ledger) sayıları üzerinden zamanı temsil ederler.

// 'DAY_IN_LEDGERS': Yaklaşık olarak bir güne denk gelen defter sayısını tanımlar.
// Stellar ağında bir defter genellikle 5 saniyede bir oluşur, bu da 24 * 60 * 60 / 5 = 17280 defter/gün anlamına gelir.
pub(crate) const DAY_IN_LEDGERS: u32 = 17280; // 'pub(crate)' bu sabitin sadece mevcut kütüphane (crate) içinde erişilebilir olduğunu belirtir.

// Kontrat örneği (instance) depolaması için TTL artırma miktarı.
// Kontratın genel durumuyla ilgili veriler (örneğin, yönetici adresi) için kullanılır.
// Burada 7 günlük defter sayısına ayarlanmış.
pub(crate) const INSTANCE_BUMP_AMOUNT: u32 = 7 * DAY_IN_LEDGERS;

// Kontrat örneği depolaması için TTL'nin artırılması gerekip gerekmediğini belirleyen eşik değer.
// Eğer bir depolama girişinin kalan TTL'si bu eşiğin altına düşerse, TTL'si 'INSTANCE_BUMP_AMOUNT' kadar artırılır.
// Genellikle 'BUMP_AMOUNT - DAY_IN_LEDGERS' olarak ayarlanır, bu da TTL'nin son gününe yaklaştığında bump yapılmasını sağlar.
pub(crate) const INSTANCE_LIFETIME_THRESHOLD: u32 = INSTANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;

// Bakiye (balance) depolaması için TTL artırma miktarı.
// Kullanıcı bakiyeleri gibi sık erişilen ve güncellenen veriler için kullanılır.
// Burada 30 günlük defter sayısına ayarlanmış.
pub(crate) const BALANCE_BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;

// Bakiye depolaması için TTL artırma eşiği.
pub(crate) const BALANCE_LIFETIME_THRESHOLD: u32 = BALANCE_BUMP_AMOUNT - DAY_IN_LEDGERS;

#[derive(Clone)] // Bu yapı (struct) için 'Clone' trait'ini otomatik olarak uygular.
                 // Bu, yapının kopyalarının oluşturulabilmesini sağlar.
#[contracttype]  // Bu yapının Soroban kontratlarında depolanabilir bir tür olduğunu belirtir.
                 // Serileştirme/deserileştirme ve diğer kontrat uyumluluk özelliklerini ekler.
pub struct AllowanceDataKey { // Harcama izni (allowance) için bir depolama anahtarı yapısı.
                              // Bir iznin kimden (from) kime (spender) verildiğini belirtir.
    pub from: Address,        // İzni veren adres.
    pub spender: Address,     // İzni kullanacak olan adres.
}

#[contracttype]  // Bu yapının Soroban kontratlarında depolanabilir bir tür olduğunu belirtir.
pub struct AllowanceValue {   // Harcama izninin (allowance) değerini tutan yapı.
    pub amount: i128,         // İzin verilen token miktarı.
    pub expiration_ledger: u32, // İznin son geçerli olacağı defter (ledger) numarası.
}

#[derive(Clone)] // Bu enum için 'Clone' trait'ini otomatik olarak uygular.
#[contracttype]  // Bu enum'ın Soroban kontratlarında depolanabilir bir tür olduğunu belirtir.
pub enum DataKey {            // Kontratın depolamasında kullanılan farklı veri türleri için anahtarları tanımlayan bir enum.
                              // Bu, depolamadaki verileri organize etmeye ve ayırt etmeye yardımcı olur.
    Allowance(AllowanceDataKey), // Bir harcama izni (allowance) verisi için anahtar. 'AllowanceDataKey' yapısını içerir.
    Balance(Address),            // Bir adresin token bakiyesi için anahtar. İlgili adresi içerir.
    Nonce(Address),              // (Bu token kontratında doğrudan kullanılmıyor gibi görünüyor ama genel bir DataKey olabilir)
                                 // Bir adres için nonce (tek kullanımlık sayı, genellikle işlem tekrarını önlemek için) değeri için anahtar.
    State(Address),              // (Bu token kontratında doğrudan kullanılmıyor gibi görünüyor ama genel bir DataKey olabilir)
                                 // Bir adresle ilişkili genel bir durum (state) verisi için anahtar.
    Admin,                       // Kontratın yönetici (administrator) adresi için anahtar. Herhangi bir veri içermez, sadece anahtarın kendisi önemlidir.
    Frozen(Address),             // Bir hesabın dondurulmuş olup olmadığını belirten durum için anahtar. İlgili adresi içerir.
}