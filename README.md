# Autoškola testy

Neoficiální grafická aplikace pro procvičování testových otázek pro žadatele o řidičské oprávnění skupiny B. Aplikace je napsána v jazyce Rust s využitím frameworku Dioxus a umožňuje spuštění jako desktopová aplikace.

## ✨ Funkce

- 📚 Procvičování otázek z oficiálních testů
- 🎯 Výběr konkrétních témat pro cílené učení  
- 🖥️ Podpora desktopového rozhraní
- 🎨 Moderní uživatelské rozhraní

## 🚀 Rychlý start

### Předpoklady

- [Rust](https://rustup.rs/) (nejnovější stabilní verze)
<!-- - [Node.js](https://nodejs.org/) a npm (pro Tailwind CSS) -->
- [Dioxus CLI](https://dioxuslabs.com/learn/0.6/getting_started) 

<!--
### Instalace závislostí

1. **Rust závislosti** (automaticky při prvním buildu)
 2. **Node.js závislosti** pro Tailwind CSS:
   ```bash
   npm install
   ``` -->

### Spuštění aplikace

```bash
# Pomocí Dioxus CLI  
dx serve --platform desktop
```

## 📁 Struktura projektu

```
src/
├── main.rs              # Hlavní soubor aplikace a routing
├── utils.rs             # Pomocné funkce
├── components/          # Komponenty UI
│   ├── mod.rs
│   ├── answer.rs        # Komponenta pro zobrazení odpovědí
│   └── test_question.rs # Komponenta pro zobrazení otázky
└── views/               # Hlavní views/stránky
    ├── mod.rs
    ├── home_view.rs     # Domovská stránka s výběrem témat
    └── question_view.rs # Stránka s otázkami
```

## 🔧 Konfigurace

### Tailwind CSS

Styling je řešen pomocí Tailwind CSS. Konfigurace je v souboru `tailwind.config.js` a hlavní styly v `input.css`.

### Dioxus

Konfigurace Dioxus je v souboru `Dioxus.toml`. Zde můžete upravit nastavení pro různé platformy.


## ⚠️ Upozornění

**TENTO PROJEKT JE NEOFICIÁLNÍ** a je určen pouze pro osobní a vzdělávací účely. Není spojen s Ministerstvem dopravy ČR ani s oficiálním portálem etesty2.mdcr.cz.

Respektujte prosím podmínky používání oficiálního webu.

## 📄 Licence  

Tento projekt je licencován pod EUPL v1.2 licencí - viz soubory `LICENSE` (v češtině) a `LICENSE_en` (v angličtině) pro podrobnosti.

## 🔗 Související projekty

- [downloader4etesty2](https://github.com/ahi6/downloader4etesty2) - Rust knihovna pro stahování otázek
- [downloader4etesty2_cli](https://github.com/ahi6/downloader4etesty2_cli) - CLI nástroj pro stahování otázek
