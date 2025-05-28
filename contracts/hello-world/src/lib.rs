// Bu dosya (lib.rs), 'soroban-token-contract' adlı Rust kütüphanesinin (crate) ana dosyasıdır.
// Kütüphanenin farklı modüllerini tanımlar ve bir araya getirir.
// Bu kütüphane, Soroban platformu için bir token (jeton) akıllı kontratı implementasyonunu içerir.
// Amacı, kontratın çeşitli bileşenlerini (yönetim, izinler, bakiye yönetimi vb.)
// ayrı modüller halinde organize etmek ve bunları tek bir kütüphane altında toplamaktır.

#![no_std] // Bu satır, Rust standart kütüphanesini kullanmadan derleme yapılacağını belirtir.
           // Genellikle gömülü sistemler veya WebAssembly (Wasm) gibi kısıtlı ortamlar için kullanılır.
           // Soroban akıllı kontratları WebAssembly olarak derlendiği için bu direktif gereklidir.

mod admin;         // 'admin' adlı modülü (ve projedeki admin.rs dosyasını) bu kütüphaneye dahil eder.
                   // Bu modül, kontratın yönetimsel işlevlerini içerir.
mod allowance;     // 'allowance' adlı modülü (ve allowance.rs dosyasını) bu kütüphaneye dahil eder.
                   // Bu modül, token harcama izinleri (allowance) ile ilgili mantığı içerir.
mod balance;       // 'balance' adlı modülü (ve balance.rs dosyasını) bu kütüphaneye dahil eder.
                   // Bu modül, kullanıcıların token bakiyelerini yönetme işlevlerini içerir.
mod contract;      // 'contract' adlı modülü (ve contract.rs dosyasını) bu kütüphaneye dahil eder.
                   // Bu modül, genellikle ana kontrat mantığını ve Soroban trait implementasyonlarını barındırır.
mod metadata;      // 'metadata' adlı modülü (ve metadata.rs dosyasını) bu kütüphaneye dahil eder.
                   // Bu modül, token'ın adı, sembolü, ondalık sayısı gibi meta verilerini yönetir.
mod storage_types; // 'storage_types' adlı modülü (ve storage_types.rs dosyasını) bu kütüphaneye dahil eder.
                   // Bu modül, kontratın depolama için kullandığı özel veri türlerini tanımlar.
mod test;          // 'test' adlı modülü (ve test.rs dosyasını) bu kütüphaneye dahil eder.
                   // Bu modül, kontratın işlevselliğini doğrulamak için birim testlerini ve entegrasyon testlerini içerir.

pub use crate::contract::TokenClient; // 'contract' modülü içinde tanımlanan 'TokenClient' adlı öğeyi
                                      // (muhtemelen bir struct veya enum) bu kütüphanenin dışından
                                      // doğrudan erişilebilir (public) hale getirir.
                                      // Bu, kütüphaneyi kullanan diğer kodların veya testlerin
                                      // 'TokenClient'a `kutuphane_adi::TokenClient` yerine
                                      // `kutuphane_adi::contract::TokenClient` yazmadan erişmesini sağlar.
                                      // Genellikle, kontratla etkileşim kurmak için bir istemci (client) arayüzü sağlar.