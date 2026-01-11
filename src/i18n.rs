// src/i18n.rs
use std::collections::HashMap;

pub fn get_translations(lang: &str) -> HashMap<&str, &str> {
    let mut map = HashMap::new();
    
    // TEXTOS COMUNES
    map.insert("author", "ANGEL A. URBINA © 2026");
    map.insert("version", "CORE v2.2");

    match lang {
        "cat" => {
            map.insert("title", "Guardià Relacional");
            map.insert("subtitle", "SISTEMA DE DEFENSA I ANÀLISI");
            map.insert("tab_system", "Anàlisi Tècnica");
            map.insert("tab_motors", "Mode Defensa (Personal)");
            map.insert("btn_text", "Escriure");
            map.insert("btn_file", "Pujar Arxiu");
            map.insert("btn_audio", "Gravar Àudio");
            map.insert("btn_analyze", "ANALITZAR RISC");
            map.insert("btn_motors", "GENERAR DEFENSA");
            map.insert("rec_start", "🔴 Gravar");
            map.insert("rec_stop", "⏹ Aturar");
            map.insert("loading", "PROCESSANT SEGURETAT...");
            map.insert("btn_help", "Manual i Ciència");
            map.insert("btn_load", "Carregar");
            map.insert("btn_exit", "Sortir");
            
            // REPORT UI
            map.insert("rep_title", "Expedient de Seguretat");
            map.insert("btn_new", "Nou Cas");
            map.insert("btn_save", "Guardar Dades");
            map.insert("btn_read", "Llegir");
            map.insert("btn_pdf", "Baixar PDF");
            map.insert("btn_html", "Baixar HTML"); // NUEVO
            map.insert("rep_confidential", "CONFIDENCIAL // NOMÉS ULLS");
            map.insert("rep_footer", "Generat per Guardià Relacional v2.2. Aquest document és una eina d'anàlisi, no un consell legal o clínic.");

            map.insert("lbl_target", "Qui et preocupa?");
            map.insert("lbl_relation", "Tipus de relació");
            map.insert("lbl_m_context", "1. El Context (Regles)");
            map.insert("ph_m_context", "Quines regles no escrites hi ha? Què passa si dius que no?");
            map.insert("lbl_m_obs", "2. Els Fets (Incidents)");
            map.insert("ph_m_obs", "Què ha passat exactament? T'han cridat? Ignorat?");
            map.insert("lbl_m_goal", "3. Objectiu");
            map.insert("ph_m_goal", "Pau mental? Marxar sense conflicte?");
            map.insert("lbl_signals", "4. Termòmetre Intern (1-5)");
            map.insert("scale_desc", "1 = Calma, 5 = Perill/Por");
            map.insert("sig_security", "Sensació de Perill");
            map.insert("sig_belonging", "Por a l'exclusió");
            map.insert("sig_status", "Vergonya / Humiliació");
            map.insert("sig_autonomy", "Asfíxia / Control");
            map.insert("sig_mastery", "Sentiment d'Inutilitat");
            map.insert("sig_justice", "Ràbia / Injustícia");
            map.insert("sig_purpose", "Confusió (Gaslighting)");
            map.insert("sig_control", "Por al Càstig");
            map.insert("sig_curiosity", "Hipervigilància (Alerta)");
            map.insert("sig_comfort", "Esgotament Físic");
            map.insert("rel_peer", "Grup Social / Amigues");
            map.insert("rel_boss", "Cap / Feina");
            map.insert("rel_team", "Companys (Mobbing)");
            map.insert("rel_client", "Parella / Sentimental");
            map.insert("rel_provider", "Grup Sectari / Religiós");
            map.insert("rel_adversary", "Professor / Entrenador");
            map.insert("rel_regulator", "Família");
            map.insert("rel_mentor", "Mentor / Autoritat");
            map.insert("rel_other", "Altre");
            map.insert("lbl_situation", "Situació");
            map.insert("ph_situation", "Descripció...");
            map.insert("lbl_cv", "Perfil");
            map.insert("ph_cv", "Dades...");
            map.insert("lbl_focus", "Focus");
            map.insert("ph_focus", "Dubtes...");
        },
        "en" => {
            map.insert("title", "Relational Guardian");
            map.insert("subtitle", "DEFENSE & ANALYSIS SYSTEM");
            map.insert("tab_system", "Technical Analysis");
            map.insert("tab_motors", "Defense Mode (Personal)");
            map.insert("btn_text", "Write");
            map.insert("btn_file", "Upload File");
            map.insert("btn_audio", "Record Audio");
            map.insert("btn_analyze", "ANALYZE RISK");
            map.insert("btn_motors", "GENERATE DEFENSE");
            map.insert("rec_start", "🔴 Record");
            map.insert("rec_stop", "⏹ Stop");
            map.insert("loading", "PROCESSING SAFETY...");
            map.insert("btn_help", "Guide & Science");
            map.insert("btn_load", "Load");
            map.insert("btn_exit", "Exit");

            // REPORT UI
            map.insert("rep_title", "Security Dossier");
            map.insert("btn_new", "New Case");
            map.insert("btn_save", "Save Data");
            map.insert("btn_read", "Listen");
            map.insert("btn_pdf", "Download PDF");
            map.insert("btn_html", "Download HTML"); // NUEVO
            map.insert("rep_confidential", "CONFIDENTIAL // EYES ONLY");
            map.insert("rep_footer", "Generated by Relational Guardian v2.2. This document is for analysis, not legal/clinical advice.");

            map.insert("lbl_target", "Who is the target?");
            map.insert("lbl_relation", "Relationship Type");
            map.insert("lbl_m_context", "1. Context (Rules)");
            map.insert("ph_m_context", "Unwritten rules? Consequences of saying 'no'?");
            map.insert("lbl_m_obs", "2. Facts (Incidents)");
            map.insert("ph_m_obs", "Specific events. Yelling? Shunning? Gaslighting?");
            map.insert("lbl_m_goal", "3. Goal");
            map.insert("ph_m_goal", "Mental peace? Safe exit? De-escalation?");
            map.insert("lbl_signals", "4. Internal Thermometer (1-5)");
            map.insert("scale_desc", "1 = Safe, 5 = Danger/Fear");
            map.insert("sig_security", "Sense of Danger");
            map.insert("sig_belonging", "Fear of Exclusion");
            map.insert("sig_status", "Shame / Humiliation");
            map.insert("sig_autonomy", "Suffocation / Control");
            map.insert("sig_mastery", "Feeling Useless");
            map.insert("sig_justice", "Rage / Injustice");
            map.insert("sig_purpose", "Confusion (Gaslighting)");
            map.insert("sig_control", "Fear of Punishment");
            map.insert("sig_curiosity", "Hypervigilance (Alert)");
            map.insert("sig_comfort", "Physical Exhaustion");
            map.insert("rel_peer", "Social Group / Friends");
            map.insert("rel_boss", "Boss / Work");
            map.insert("rel_team", "Peers (Mobbing)");
            map.insert("rel_client", "Partner / Romantic");
            map.insert("rel_provider", "Cult / Religious Group");
            map.insert("rel_adversary", "Teacher / Coach");
            map.insert("rel_regulator", "Family");
            map.insert("rel_mentor", "Mentor / Authority");
            map.insert("rel_other", "Other");
            map.insert("lbl_situation", "Situation");
            map.insert("ph_situation", "Description...");
            map.insert("lbl_cv", "Profile");
            map.insert("ph_cv", "Background...");
            map.insert("lbl_focus", "Focus");
            map.insert("ph_focus", "Specifics...");
        },
        _ => { // ES
            map.insert("title", "Guardián Relacional");
            map.insert("subtitle", "SISTEMA DE DEFENSA Y ANÁLISIS");
            map.insert("tab_system", "Análisis Técnico");
            map.insert("tab_motors", "Modo Defensa (Personal)");
            map.insert("btn_text", "Escribir");
            map.insert("btn_file", "Subir Archivo");
            map.insert("btn_audio", "Grabar Audio");
            map.insert("btn_analyze", "ANALIZAR RIESGO");
            map.insert("btn_motors", "GENERAR DEFENSA");
            map.insert("rec_start", "🔴 Grabar");
            map.insert("rec_stop", "⏹ Detener");
            map.insert("loading", "PROCESANDO SEGURIDAD...");
            map.insert("btn_help", "Manual y Ciencia");
            map.insert("btn_load", "Cargar");
            map.insert("btn_exit", "Salir");

            // REPORT UI
            map.insert("rep_title", "Expediente de Seguridad");
            map.insert("btn_new", "Nuevo Caso");
            map.insert("btn_save", "Guardar Datos");
            map.insert("btn_read", "Leer en Voz Alta");
            map.insert("btn_pdf", "Descargar PDF");
            map.insert("btn_html", "Descargar HTML"); // NUEVO
            map.insert("rep_confidential", "CONFIDENCIAL // SOLO OJOS");
            map.insert("rep_footer", "Generado por Guardián Relacional v2.2. Este documento es una herramienta de análisis, no consejo legal o clínico.");

            map.insert("lbl_target", "¿Quién te preocupa?");
            map.insert("lbl_relation", "Tipo de relación");
            map.insert("lbl_m_context", "1. El Contexto (Reglas)");
            map.insert("ph_m_context", "¿Qué reglas no escritas hay? ¿Qué pasa si dices 'no'?");
            map.insert("lbl_m_obs", "2. Los Hechos (Incidentes)");
            map.insert("ph_m_obs", "¿Qué ocurrió? ¿Gritos? ¿Ley del hielo? ¿Culpa?");
            map.insert("lbl_m_goal", "3. Objetivo");
            map.insert("ph_m_goal", "¿Paz mental? ¿Salir sin conflicto? ¿Entender?");
            map.insert("lbl_signals", "4. Termómetro Interno (1-5)");
            map.insert("scale_desc", "1 = Calma, 5 = Peligro/Miedo");
            map.insert("sig_security", "Sensación de Peligro");
            map.insert("sig_belonging", "Miedo a Exclusión");
            map.insert("sig_status", "Vergüenza / Humillación");
            map.insert("sig_autonomy", "Asfixia / Control");
            map.insert("sig_mastery", "Sensación de Inutilidad");
            map.insert("sig_justice", "Rabia / Injusticia");
            map.insert("sig_purpose", "Confusión (Gaslighting)");
            map.insert("sig_control", "Miedo al Castigo");
            map.insert("sig_curiosity", "Hipervigilancia (Alerta)");
            map.insert("sig_comfort", "Agotamiento Físico");
            map.insert("rel_peer", "Grupo Social / Amigas");
            map.insert("rel_boss", "Jefe / Trabajo");
            map.insert("rel_team", "Compañeros (Mobbing)");
            map.insert("rel_client", "Pareja / Sentimental");
            map.insert("rel_provider", "Grupo Sectario / Religioso");
            map.insert("rel_adversary", "Profesor / Entrenador");
            map.insert("rel_regulator", "Familia");
            map.insert("rel_mentor", "Mentor / Autoridad");
            map.insert("rel_other", "Otro");
            map.insert("lbl_situation", "Situación");
            map.insert("ph_situation", "Descripción...");
            map.insert("lbl_cv", "Perfil");
            map.insert("ph_cv", "Antecedentes...");
            map.insert("lbl_focus", "Foco");
            map.insert("ph_focus", "¿Dudas concretas?");
        }
    };
    map
}