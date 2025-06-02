# Soroban Token Kontratı (soroban-token-contract)

Bu proje, [Stellar](https://stellar.org/) ağı üzerinde çalışan [Soroban](https://soroban.stellar.org/) akıllı kontrat platformu için geliştirilmiş bir token (jeton) kontratıdır.
Standart token işlevselliklerini (transfer, bakiye sorgulama, onay mekanizmaları vb.) ve yöneticiye özel bazı ek özellikleri içerir.

## Projenin Amacı

Bu kontrat, aşağıdaki amaçlar için bir temel veya örnek olarak kullanılabilir:

*   Soroban üzerinde özel bir token oluşturmak.
*   `soroban-sdk` ve `soroban-token-sdk` kullanarak token geliştirme pratiklerini öğrenmek.
*   Yönetici kontrollü özelliklere (token basma, hesap dondurma, token geri alma) sahip bir token altyapısı sağlamak.

## Temel Özellikler

*   **Token Başlatma (`initialize`):** Token'ı ondalık basamak sayısı, adı, sembolü ve bir yönetici adresi ile başlatır.
*   **Standart Token Fonksiyonları:**
    *   `balance`: Bir adresin token bakiyesini sorgular.
    *   `transfer`: Belirli bir adresten diğerine token transfer eder.
    *   `approve`: Bir harcayıcının (spender) belirli bir miktar token harcamasına izin verir.
    *   `allowance`: Bir sahibin (owner) bir harcayıcıya ne kadar token harcama izni verdiğini sorgular.
    *   `transfer_from`: Onaylanmış bir miktarı bir adresten diğerine transfer eder.
    *   `burn`: Belirli bir adresten token yakar (yok eder).
    *   `burn_from`: Onaylanmış bir miktarı belirli bir adresten yakar.
*   **Yönetici (Admin) Fonksiyonları:**
    *   `mint`: Belirli bir adrese yeni token basar (sadece yönetici).
    *   `set_admin`: Kontratın yöneticisini değiştirir (sadece mevcut yönetici).
    *   `set_authorized`: Bir hesabın token transferi yapabilme durumunu ayarlar (dondurma/çözme, sadece yönetici).
    *   `clawback`: Belirli bir hesaptan tokenları geri alır (sadece yönetici).
*   **Metaveri Fonksiyonları:**
    *   `decimals`: Token'ın ondalık basamak sayısını döndürür.
    *   `name`: Token'ın adını döndürür.
    *   `symbol`: Token'ın sembolünü döndürür.

### Kontratı Derleme

```bash
soroban contract build
```
### Testeleri Çalıştırma
```bash
cargo test
```

*The code in this project has been interpreted with the help of AI.*
