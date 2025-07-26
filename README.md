# Autoškola testy

Neoficiální grafická aplikace pro procvičování testových otázek pro žadatele o řidičské oprávnění skupiny B. Aplikace je napsána v jazyce Rust s využitím frameworku Dioxus a umožňuje spuštění jako desktopová aplikace.

## ✨ Funkce

- 📚 Procvičování otázek z oficiálních testů
- 🎯 Výběr konkrétních témat pro cílené učení  
- 🖥️ Podpora desktopového rozhraní
- 🎨 Moderní uživatelské rozhraní

## Snímky obrazovky
<table>
  <tr>
    <th>Uvítací obrazovka</th>
    <th>Výběr oblasti k procvičení</th>
    <th>Otázka</th>
  </tr>
  <tr>
    <td>
      <img width="300" alt="Welcome screen" src="https://github.com/user-attachments/assets/f0ff61fb-3983-4bff-ab65-03ca55596a33" />
    </td>
    <td>
      <img width="300" alt="Topic select screen" src="https://github.com/user-attachments/assets/34cba36e-cba2-4be1-9ae3-ead9301ff11b" />
    </td>
    <td>
      <img width="300" alt="Question screen" src="https://github.com/user-attachments/assets/c93f8d0d-28f6-4351-8ae7-80501d78a255" />
    </td>
  </tr>
</table>


## 🚀 Rychlý start

### Předpoklady

- [Rust](https://rustup.rs/) (nejnovější stabilní verze)
<!-- - [Node.js](https://nodejs.org/) a npm (pro Tailwind CSS) -->
- [Dioxus CLI](https://dioxuslabs.com/learn/0.6/getting_started) 

### Stažení zdrojového kódu

```bash
git clone https://github.com/ahi6/autoskola_testy_workspace
cd autoskola_testy_workspace/autoskola_testy
```

### Spuštění aplikace

```bash
# Pomocí Dioxus CLI  
dx serve --platform desktop
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
- https://github.com/ahi6/autoskola_testy_workspace - Rust workspace se všemi souvisejícími repozitáři
