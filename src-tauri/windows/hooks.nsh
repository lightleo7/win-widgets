!macro NSIS_HOOK_POSTINSTALL

CopyFiles /SILENT "$INSTDIR\resources\WebView2Loader.dll" "$INSTDIR\WebView2Loader.dll"

!macroend

!macro NSIS_HOOK_PREUNINSTALL

Delete "$INSTDIR\resources\WebView2Loader.dll"

!macroend
