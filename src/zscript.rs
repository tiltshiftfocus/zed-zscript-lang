use zed_extension_api as zed;

struct ZScriptLang {
    // ... state
}

impl zed::Extension for ZScriptLang {
    // ...
}

zed::register_extension!(ZScriptLang);
