use windows::Win32::{
    Foundation::{BOOL, HWND, LPARAM},
    UI::WindowsAndMessaging::{EnumWindows, GetWindowTextA, GetWindowTextLengthA, IsWindowVisible, ShowWindow, SW_HIDE, SW_SHOW},
};

fn main() {
    let title_suffix = "Obsidian v1.7.7"; // Sufijo exacto del título a buscar

    unsafe {
        match find_window_by_suffix(title_suffix) {
            Some((hwnd, is_visible)) => {
                println!(
                    "Ventana encontrada: HWND: {:?}, Visible: {}",
                    hwnd.0, is_visible
                );
                if toggle_window_visibility(hwnd) {
                    println!("La visibilidad de la ventana fue alternada con éxito.");
                } else {
                    println!("No se pudo alternar la visibilidad de la ventana.");
                }
            }
            None => {
                println!(
                    "No se encontró ninguna ventana cuyo título termine con '{}'.",
                    title_suffix
                );
                println!("=== Lista de Ventanas Visibles y Ocultas ===");
                list_all_windows();
                println!("=== Fin de la lista ===");
            }
        }
    }
}

/// Lista todas las ventanas visibles y ocultas en el sistema.
unsafe fn list_all_windows() {
    EnumWindows(Some(enum_windows_proc), LPARAM(0));

    extern "system" fn enum_windows_proc(hwnd: HWND, _: LPARAM) -> BOOL {
        unsafe {
            let length = GetWindowTextLengthA(hwnd) as usize;
            if length > 0 {
                let mut buffer = vec![0u8; length + 1]; // +1 para el carácter nulo

                if GetWindowTextA(hwnd, &mut buffer) > 0 {
                    let window_title = String::from_utf8_lossy(&buffer).trim_end_matches('\0').to_string();
                    let is_visible = IsWindowVisible(hwnd).as_bool();

                    println!(
                        "HWND: {:?}, Título: \"{}\", Visible: {}",
                        hwnd.0, window_title, is_visible
                    );
                }
            }
            BOOL(1) // Continuar con la siguiente ventana
        }
    }
}

/// Busca una ventana cuyo título termina con `title_suffix` y devuelve su HWND y visibilidad.
unsafe fn find_window_by_suffix(title_suffix: &str) -> Option<(HWND, bool)> {
    let mut result: Option<(HWND, bool)> = None;

    let callback_data = (&mut result as *mut _, title_suffix as *const _);
    EnumWindows(Some(enum_windows_proc_find), LPARAM(&callback_data as *const _ as isize));

    result
}

unsafe extern "system" fn enum_windows_proc_find(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let (result_ptr, suffix_ptr): (*mut Option<(HWND, bool)>, &str) =
        *(lparam.0 as *const (*mut Option<(HWND, bool)>, &str));

    let length = GetWindowTextLengthA(hwnd) as usize;
    if length > 0 {
        let mut buffer = vec![0u8; length + 1]; // +1 para el carácter nulo

        if GetWindowTextA(hwnd, &mut buffer) > 0 {
            let window_title = String::from_utf8_lossy(&buffer).trim_end_matches('\0').to_string();

            let suffix = suffix_ptr;

            if window_title.ends_with(suffix) {
                let is_visible = IsWindowVisible(hwnd).as_bool();
                *result_ptr = Some((hwnd, is_visible));
                return BOOL(0); // Detener la enumeración
            }
        }
    }

    BOOL(1) // Continuar con la siguiente ventana
}

/// Alterna la visibilidad de una ventana (ocultar si está visible, mostrar si está oculta).
unsafe fn toggle_window_visibility(hwnd: HWND) -> bool {
    if IsWindowVisible(hwnd).as_bool() {
        println!("La ventana está visible. Ocultándola...");
        ShowWindow(hwnd, SW_HIDE).as_bool()
    } else {
        println!("La ventana está oculta. Mostrándola...");
        ShowWindow(hwnd, SW_SHOW).as_bool()
    }
}
