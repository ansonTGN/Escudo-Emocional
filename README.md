# Relational Guardian & Systemic Analyst (Core v2.2)

![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange?style=flat-square&logo=rust)
![Actix-Web](https://img.shields.io/badge/Actix--Web-v4-green?style=flat-square)
![OpenAI Whisper](https://img.shields.io/badge/AI-Whisper%20%2B%20GPT--4o-blue?style=flat-square&logo=openai)
![License](https://img.shields.io/badge/License-MIT-lightgrey?style=flat-square)

**[ 🇪🇸 Español ](#-español) | [ 🇺🇸 English ](#-english) | [ 🏴󠁥󠁳󠁣󠁴󠁿 Català ](#-català)**

> **Author / Autor:** Angel A. Urbina  
> **Version:** Core v2.2 (2026)

---

<a name="-español"></a>
## 🇪🇸 Español

### 🧠 Introducción para Psicólogos y Terapeutas

**Relational Guardian** no es un simple chatbot. Es una herramienta de **Inteligencia Operativa basada en el Trauma (Trauma-Informed Intelligence)** diseñada para estructurar el relato caótico que suelen presentar las víctimas de abuso psicológico, control coercitivo o entornos organizacionales tóxicos.

El software actúa como un **triaje automatizado** que ayuda al usuario a validar su realidad mediante la operacionalización de modelos clínicos validados:

1.  **Modelo BITE (Steven Hassan):** El sistema analiza el texto y el audio buscando patrones específicos de control sobre la Conducta, Información, Pensamiento y Emociones, distinguiendo entre una relación sana y una dinámica sectaria o coercitiva.
2.  **Teoría Polivagal (Stephen Porges & Deb Dana):** La interfaz solicita al usuario puntuar sus "Sensores Somáticos" (hipervigilancia, congelación, confusión). El sistema interpreta estos datos no como "síntomas patológicos", sino como respuestas adaptativas de *neurocepción* ante una amenaza relacional.
3.  **Seguridad Psicológica (Amy Edmondson):** Se utiliza para diferenciar entornos de alto rendimiento (donde el error es aprendizaje) de entornos de indefensión aprendida (donde el error se castiga con humillación).
4.  **Neurociencia de la Exclusión (Kipling Williams):** Evalúa el impacto del ostracismo ("ley del hielo") como una forma de agresión física a nivel neural.

**Objetivo Clínico:** Reducir la disonancia cognitiva de la víctima, validar su percepción de riesgo y ofrecer estrategias de protección (como la técnica de "Piedra Gris") antes de que llegue a consulta.

---

### 📋 Descripción Técnica

Esta aplicación es una suite de defensa personal y análisis sistémico escrita en **Rust**. Permite a los usuarios documentar situaciones conflictivas mediante texto, archivos o **notas de voz** (transcritas automáticamente con Whisper).

#### Funcionalidades Clave
*   **Modo Defensa (Guardián):** Para víctimas de mobbing, relaciones tóxicas o dinámicas de exclusión. Genera un "Escudo" con estrategias de validación y límites.
*   **Modo Análisis (Técnico):** Para consultores que necesitan evaluar riesgos sistémicos usando marcos PESTEL y análisis de incentivos.
*   **Semáforo de Riesgo:** Clasificación automática de la situación en Verde (Sano), Amarillo (Tóxico) o Rojo (Coercitivo).
*   **Privacidad Local:** Los datos sensibles se procesan en memoria y se pueden exportar a JSON/PDF para custodia del usuario, sin bases de datos persistentes que comprometan la privacidad.

### 🛠️ Instalación y Uso

**Requisitos Previos:**
*   Rust (Cargo)
*   Librerías del sistema: `pkg-config`, `libssl-dev`
*   Una API Key de OpenAI

**Configuración (`.env`):**
```env
OPENAI_API_KEY=sk-tu-clave-aqui
OPENAI_MODEL=gpt-4o-mini
PORT=8080
BIND_HOST=0.0.0.0
```

**Ejecutar:**
```bash
cargo run
```
Accede a `http://localhost:8080`.

---

<a name="-english"></a>
## 🇺🇸 English

### 🧠 Introduction for Psychologists & Therapists

**Relational Guardian** is not just a chatbot. It is a **Trauma-Informed Operational Intelligence tool** designed to structure the often chaotic narratives presented by victims of psychological abuse, coercive control, or toxic organizational environments.

The software acts as an **automated triage system** that helps users validate their reality by operationalizing validated clinical models:

1.  **BITE Model (Steven Hassan):** The system analyzes text and audio input looking for specific patterns of control over Behavior, Information, Thought, and Emotions, distinguishing between healthy relationships and sectarian/coercive dynamics.
2.  **Polyvagal Theory (Stephen Porges & Deb Dana):** The interface asks users to rate their "Somatic Sensors" (hypervigilance, freeze response, confusion). The system interprets this data not as "pathological symptoms," but as adaptive *neuroception* responses to relational threats.
3.  **Psychological Safety (Amy Edmondson):** Used to differentiate high-performance environments (where mistakes are learning opportunities) from learned helplessness environments (where mistakes are punished with humiliation).
4.  **Neuroscience of Ostracism (Kipling Williams):** Evaluates the impact of the "silent treatment" as a form of physical aggression at a neural level.

**Clinical Goal:** To reduce the victim's cognitive dissonance, validate their perception of risk, and offer protection strategies (such as the "Gray Rock" method) before they reach professional therapy.

---

### 📋 Technical Overview

This application is a personal defense and systemic analysis suite built in **Rust**. It allows users to document conflict situations via text, files, or **voice notes** (automatically transcribed via Whisper).

#### Key Features
*   **Defense Mode (Guardian):** For victims of mobbing, toxic relationships, or exclusion dynamics. Generates a "Shield" with validation and boundary strategies.
*   **Analysis Mode (Technical):** For consultants needing to assess systemic risks using PESTEL frameworks and incentive analysis.
*   **Risk Traffic Light:** Automatic classification of the situation into Green (Safe), Yellow (Toxic), or Red (Coercive).
*   **Local Privacy:** Sensitive data is processed in memory and can be exported to JSON/PDF for user custody, with no persistent databases compromising privacy.

### 🛠️ Installation & Usage

**Prerequisites:**
*   Rust (Cargo)
*   System libraries: `pkg-config`, `libssl-dev`
*   OpenAI API Key

**Configuration (`.env`):**
```env
OPENAI_API_KEY=sk-your-key-here
OPENAI_MODEL=gpt-4o-mini
PORT=8080
BIND_HOST=0.0.0.0
```

**Run:**
```bash
cargo run
```
Access at `http://localhost:8080`.

---

<a name="-català"></a>
## 🏴󠁥󠁳󠁣󠁴󠁿 Català

### 🧠 Introducció per a Psicòlegs i Terapeutes

**Guardià Relacional** no és un simple xatbot. És una eina d'**Intel·ligència Operativa basada en el Trauma (Trauma-Informed Intelligence)** dissenyada per estructurar el relat caòtic que solen presentar les víctimes d'abús psicològic, control coercitiu o entorns organitzacionals tòxics.

El programari actua com un **triatge automatitzat** que ajuda l'usuari a validar la seva realitat mitjançant l'operacionalització de models clínics validats:

1.  **Model BITE (Steven Hassan):** El sistema analitza el text i l'àudio cercant patrons específics de control sobre la Conducta, Informació, Pensament i Emocions, distingint entre una relació sana i una dinàmica sectària o coercitiva.
2.  **Teoria Polivagal (Stephen Porges & Deb Dana):** La interfície sol·licita a l'usuari puntuar els seus "Sensors Somàtics" (hipervigilància, congelació, confusió). El sistema interpreta aquestes dades no com a "símptomes patològics", sinó com a respostes adaptatives de *neurocepció* davant d'una amenaça relacional.
3.  **Seguretat Psicològica (Amy Edmondson):** S'utilitza per diferenciar entorns d'alt rendiment (on l'error és aprenentatge) d'entorns d'indefensió apresa (on l'error es castiga amb humiliació).
4.  **Neurociència de l'Exclusió (Kipling Williams):** Avalua l'impacte de l'ostracisme ("fer el buit") com una forma d'agressió física a nivell neural.

**Objectiu Clínic:** Reduir la dissonància cognitiva de la víctima, validar la seva percepció de risc i oferir estratègies de protecció (com la tècnica de "Pedra Grisa") abans que arribi a consulta.

---

### 📋 Descripció Tècnica

Aquesta aplicació és una suite de defensa personal i anàlisi sistèmica escrita en **Rust**. Permet als usuaris documentar situacions conflictives mitjançant text, arxius o **notes de veu** (transcrites automàticament amb Whisper).

#### Funcionalitats Clau
*   **Mode Defensa (Guardià):** Per a víctimes de mobbing, relacions tòxiques o dinàmiques d'exclusió. Genera un "Escut" amb estratègies de validació i límits.
*   **Mode Anàlisi (Tècnic):** Per a consultors que necessiten avaluar riscos sistèmics usant marcs PESTEL i anàlisi d'incentius.
*   **Semàfor de Risc:** Classificació automàtica de la situació en Verd (Sa), Groc (Tòxic) o Vermell (Coercitiu).
*   **Privadesa Local:** Les dades sensibles es processen en memòria i es poden exportar a JSON/PDF per a custòdia de l'usuari, sense bases de dades persistents que comprometin la privadesa.

### 🛠️ Instal·lació i Ús

**Requisits Previs:**
*   Rust (Cargo)
*   Llibreries del sistema: `pkg-config`, `libssl-dev`
*   Una API Key d'OpenAI

**Configuració (`.env`):**
```env
OPENAI_API_KEY=sk-la-teva-clau-aqui
OPENAI_MODEL=gpt-4o-mini
PORT=8080
BIND_HOST=0.0.0.0
```

**Executar:**
```bash
cargo run
```
Accedeix a `http://localhost:8080`.

---

## ⚠️ Disclaimer / Avís Legal

**ES:** Este software es una herramienta de análisis y educación. **No sustituye el consejo médico, psicológico o legal profesional.** Si estás en peligro físico inmediato, contacta con los servicios de emergencia de tu país.

**EN:** This software is an analysis and educational tool. **It does not substitute professional medical, psychological, or legal advice.** If you are in immediate physical danger, please contact your local emergency services.

**CAT:** Aquest programari és una eina d'anàlisi i educació. **No substitueix el consell mèdic, psicològic o legal professional.** Si estàs en perill físic immediat, contacta amb els serveis d'emergència del teu país.

---

## License

This project is licensed under the [MIT License](LICENSE).

**© 2026 Angel A. Urbina. All Rights Reserved.**
