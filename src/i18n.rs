pub const LANGUAGES: [(&str, &str); 8] = [
    ("fr", "Français"),
    ("en", "English"),
    ("es", "Español"),
    ("de", "Deutsch"),
    ("it", "Italiano"),
    ("pt", "Português"),
    ("zh-CN", "简体中文"),
    ("ja", "日本語"),
];

pub fn t(lang: &str, key: &str) -> &'static str {
    let i = LANGUAGES
        .iter()
        .position(|(code, _)| *code == lang)
        .unwrap_or(0);
    let row: &[&str; 8] = match key {
        "devices" => &["Périphériques", "Devices", "Dispositivos", "Geräte", "Dispositivi", "Dispositivos", "设备", "デバイス"],
        "refresh" => &["Actualiser", "Refresh", "Actualizar", "Aktualisieren", "Aggiorna", "Atualizar", "刷新", "更新"],
        "profiles" => &["Profils", "Profiles", "Perfiles", "Profile", "Profili", "Perfis", "配置文件", "プロファイル"],
        "add" => &["Ajouter", "Add", "Añadir", "Hinzufügen", "Aggiungi", "Adicionar", "添加", "追加"],
        "delete" => &["Supprimer", "Delete", "Eliminar", "Löschen", "Elimina", "Excluir", "删除", "削除"],
        "save" => &["Enregistrer", "Save", "Guardar", "Speichern", "Salva", "Salvar", "保存", "保存"],
        "apply" => &["Appliquer au périphérique", "Apply to device", "Aplicar al dispositivo", "Auf Gerät anwenden", "Applica al dispositivo", "Aplicar ao dispositivo", "应用到设备", "デバイスに適用"],
        "name" => &["Nom", "Name", "Nombre", "Name", "Nome", "Nome", "名称", "名前"],
        "rate" => &["Intervalle (ms)", "Report interval (ms)", "Intervalo (ms)", "Abfrageintervall (ms)", "Intervallo (ms)", "Intervalo (ms)", "报告间隔 (毫秒)", "レポート間隔 (ms)"],
        "color" => &["Couleur RVB", "RGB color", "Color RGB", "RGB-Farbe", "Colore RGB", "Cor RGB", "RGB 颜色", "RGB 色"],
        "notes" => &["Notes", "Notes", "Notas", "Notizen", "Note", "Notas", "备注", "メモ"],
        "status" => &["État", "Status", "Estado", "Status", "Stato", "Estado", "状态", "状態"],
        "no_device" => &["Aucun périphérique Logitech détecté", "No Logitech device detected", "No se detectó ningún dispositivo Logitech", "Kein Logitech-Gerät erkannt", "Nessun dispositivo Logitech rilevato", "Nenhum dispositivo Logitech detectado", "未检测到 Logitech 设备", "Logitech デバイスが見つかりません"],
        "linux_note" => &["Sous Linux, ratbagd permet d'appliquer DPI et intervalle aux souris compatibles. Les autres fonctions matérielles sont en développement.", "On Linux, ratbagd can apply DPI and report interval to compatible mice. Other hardware features are in development.", "En Linux, ratbagd aplica DPI e intervalo a ratones compatibles. Otras funciones están en desarrollo.", "Unter Linux kann ratbagd DPI und Intervall für kompatible Mäuse setzen. Weitere Hardwarefunktionen folgen.", "Su Linux, ratbagd applica DPI e intervallo ai mouse compatibili. Altre funzioni sono in sviluppo.", "No Linux, ratbagd aplica DPI e intervalo a mouses compatíveis. Outras funções estão em desenvolvimento.", "Linux 可通过 ratbagd 设置兼容鼠标的 DPI 和报告间隔。其他硬件功能仍在开发中。", "Linux では ratbagd で対応マウスの DPI とレポート間隔を設定できます。その他の機能は開発中です。"],
        "desktop_note" => &["La détection USB fonctionne. La configuration matérielle sur ce système est en développement.", "USB discovery works. Hardware configuration on this system is in development.", "La detección USB funciona. La configuración de hardware está en desarrollo.", "USB-Erkennung funktioniert. Hardwarekonfiguration ist in Entwicklung.", "Il rilevamento USB funziona. La configurazione hardware è in sviluppo.", "A detecção USB funciona. A configuração de hardware está em desenvolvimento.", "USB 检测可用。此系统上的硬件配置仍在开发中。", "USB 検出は利用できます。このシステムでのハードウェア設定は開発中です。"],
        _ => &["", "", "", "", "", "", "", ""],
    };
    row[i]
}
