const translations: Record<string, Record<string, string>> = {
    en: {
        nearby_devices: 'Nearby Devices',
        send_top_item: 'Send Top Item',
        compress_image: 'Compress Image',
        zip_file: 'Zip File',
        clean_url: 'Clean URL',
        generate_qr: 'Generate QR',
        remove: 'Remove',
        enter_pin: 'Enter 4-digit PIN for',
        sent_successfully: 'Sent successfully!',
        failed: 'Failed:',
        no_items: 'No items in Stash to send!',
        pin: 'PIN'
    },
    ru: {
        nearby_devices: 'Устройства рядом',
        send_top_item: 'Отправить верхний элемент',
        compress_image: 'Сжать картинку',
        zip_file: 'Сжать в ZIP',
        clean_url: 'Очистить ссылку',
        generate_qr: 'Создать QR-код',
        remove: 'Удалить',
        enter_pin: 'Введите 4-значный PIN для',
        sent_successfully: 'Успешно отправлено!',
        failed: 'Ошибка:',
        no_items: 'Нет элементов для отправки!',
        pin: 'ПИН'
    },
    es: {
        nearby_devices: 'Dispositivos cercanos',
        send_top_item: 'Enviar elemento superior',
        compress_image: 'Comprimir imagen',
        zip_file: 'Comprimir en ZIP',
        clean_url: 'Limpiar URL',
        generate_qr: 'Generar código QR',
        remove: 'Eliminar',
        enter_pin: 'Ingrese el PIN de 4 dígitos para',
        sent_successfully: '¡Enviado con éxito!',
        failed: 'Falló:',
        no_items: '¡No hay elementos en Stash para enviar!',
        pin: 'PIN'
    },
    fr: {
        nearby_devices: 'Appareils à proximité',
        send_top_item: 'Envoyer l\'élément supérieur',
        compress_image: 'Compresser l\'image',
        zip_file: 'Fichier Zip',
        clean_url: 'Nettoyer l\'URL',
        generate_qr: 'Générer un QR',
        remove: 'Supprimer',
        enter_pin: 'Entrez le code PIN à 4 chiffres pour',
        sent_successfully: 'Envoyé avec succès !',
        failed: 'Échoué :',
        no_items: 'Aucun élément dans Stash à envoyer !',
        pin: 'PIN'
    },
    de: {
        nearby_devices: 'Geräte in der Nähe',
        send_top_item: 'Oberstes Element senden',
        compress_image: 'Bild komprimieren',
        zip_file: 'ZIP-Datei',
        clean_url: 'URL bereinigen',
        generate_qr: 'QR generieren',
        remove: 'Entfernen',
        enter_pin: 'Geben Sie die 4-stellige PIN ein für',
        sent_successfully: 'Erfolgreich gesendet!',
        failed: 'Fehlgeschlagen:',
        no_items: 'Keine Elemente in Stash zum Senden!',
        pin: 'PIN'
    },
    zh: {
        nearby_devices: '附近设备',
        send_top_item: '发送顶部项目',
        compress_image: '压缩图片',
        zip_file: '压缩文件',
        clean_url: '清理链接',
        generate_qr: '生成二维码',
        remove: '移除',
        enter_pin: '输入4位PIN码发送给',
        sent_successfully: '发送成功！',
        failed: '失败：',
        no_items: 'Stash中没有可发送的项目！',
        pin: 'PIN'
    },
    ja: {
        nearby_devices: '近くのデバイス',
        send_top_item: '一番上のアイテムを送信',
        compress_image: '画像を圧縮',
        zip_file: 'ZIPファイル',
        clean_url: 'URLをクリーンアップ',
        generate_qr: 'QRを生成',
        remove: '削除',
        enter_pin: 'の4桁のPINを入力してください',
        sent_successfully: '送信に成功しました！',
        failed: '失敗：',
        no_items: '送信するアイテムがStashにありません！',
        pin: 'PIN'
    }
};

let currentLocale = $state('en');

if (typeof window !== 'undefined') {
    const navLang = navigator.language.split('-')[0].toLowerCase();
    if (translations[navLang]) {
        currentLocale = navLang;
    }
}

export function t(key: string): string {
    return translations[currentLocale]?.[key] || translations['en'][key] || key;
}

export function setLocale(lang: string) {
    if (translations[lang]) {
        currentLocale = lang;
    }
}
