#![windows_subsystem = "windows"]

use std::time::Duration;
use windows::Win32::{
    Foundation::{BOOL, HWND, LPARAM},
    UI::WindowsAndMessaging::{
        EnumWindows, GetWindowTextA, GetWindowTextLengthA, IsWindowVisible, ShowWindow,
        SW_HIDE, SetForegroundWindow, SW_RESTORE, SW_MAXIMIZE, BringWindowToTop, SetWindowPos,
        HWND_TOPMOST, SWP_NOMOVE, SWP_NOSIZE, SWP_SHOWWINDOW, GetForegroundWindow, GetWindowThreadProcessId,
    },
    System::Threading::{AttachThreadInput, GetCurrentThreadId},
};

fn main() {
    let title_suffix = "Obsidian v1.7.7"; // Sufijo exacto del título a buscar

    unsafe {
        if let Some((hwnd, is_visible)) = find_window_by_suffix(title_suffix) {
            println!(
                "Ventana encontrada: HWND: {:?}, Visible: {}",
                hwnd.0, is_visible
            );
            if toggle_window_visibility(hwnd) {
                println!("La visibilidad de la ventana fue alternada con éxito.");
            } else {
                println!("No se pudo alternar la visibilidad de la ventana.");
            }
        } else {
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

/// Lista todas las ventanas visibles y ocultas en el sistema.
unsafe fn list_all_windows() {
    EnumWindows(Some(enum_windows_proc), LPARAM(0)).ok();

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
    EnumWindows(Some(enum_windows_proc_find), LPARAM(&callback_data as *const _ as isize)).ok();

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

/// Alterna la visibilidad de una ventana (mostrar y superponer si está oculta, ocultar si está visible).
unsafe fn toggle_window_visibility(hwnd: HWND) -> bool {
    if IsWindowVisible(hwnd).as_bool() {
        println!("La ventana está visible. Ocultándola...");
        return ShowWindow(hwnd, SW_HIDE).as_bool();
    }

    println!("La ventana está oculta. Intentando mostrarla...");

    ShowWindow(hwnd, SW_RESTORE).ok(); // Restaurar la ventana
    println!("Esperando que la ventana se vuelva visible...");

    for _ in 0..100 { // Máximo de 1 segundo (100 * 10ms)
        if IsWindowVisible(hwnd).as_bool() {
            println!("La ventana ahora está visible. Intentando maximizar...");
            ShowWindow(hwnd, SW_MAXIMIZE).ok(); // Maximizar la ventana

            let foreground_window = GetForegroundWindow();
            let current_thread_id = GetCurrentThreadId();
            let foreground_thread_id =
                GetWindowThreadProcessId(foreground_window, Some(std::ptr::null_mut()));

            AttachThreadInput(foreground_thread_id, current_thread_id, true).ok();
            let set_foreground_result = SetForegroundWindow(hwnd).as_bool();
            AttachThreadInput(foreground_thread_id, current_thread_id, false).ok();

            if set_foreground_result {
                println!("La ventana fue superpuesta y maximizada con éxito.");
                return true;
            } else {
                println!("El intento de superponer la ventana falló. Intentando métodos alternativos...");
                BringWindowToTop(hwnd).ok();
                return SetWindowPos(
                    hwnd,
                    HWND_TOPMOST,
                    0,
                    0,
                    0,
                    0,
                    SWP_NOMOVE | SWP_NOSIZE | SWP_SHOWWINDOW,
                )
                    .is_ok();
            }
        }
        std::thread::sleep(Duration::from_millis(1));
    }

    println!("La ventana sigue sin ser visible después del tiempo de espera.");
    false
}
