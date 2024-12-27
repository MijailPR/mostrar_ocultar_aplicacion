use windows::Win32::{
    Foundation::{HWND},
    UI::WindowsAndMessaging::{
        SetForegroundWindow, ShowWindow, SW_RESTORE, BringWindowToTop, SwitchToThisWindow,
        GetForegroundWindow, GetWindowThreadProcessId, SetWindowPos, HWND_TOPMOST, SWP_NOMOVE, SWP_NOSIZE, SWP_SHOWWINDOW,
    },
    System::Threading::{AttachThreadInput, GetCurrentThreadId},
};

fn main() {
    unsafe {
        // Reemplaza el valor con el HWND correcto
        let hwnd = HWND(0x40756 as *mut _); // HWND de la ventana objetivo
        if force_foreground_window(hwnd) {
            println!("Ventana colocada en primer plano y superpuesta con éxito.");
        } else {
            println!("No se pudo colocar la ventana en primer plano y superponerla.");
        }
    }
}

/// Fuerza que una ventana se coloque en primer plano utilizando AttachThreadInput, BringWindowToTop y SetWindowPos.
unsafe fn force_foreground_window(hwnd: HWND) -> bool {
    let foreground_window = GetForegroundWindow();
    if foreground_window.0 == std::ptr::null_mut() {
        return false;
    }

    let current_thread_id = GetCurrentThreadId();
    let foreground_thread_id = GetWindowThreadProcessId(foreground_window, Some(std::ptr::null_mut()));

    // Conectar los hilos de entrada
    let _ = AttachThreadInput(foreground_thread_id, current_thread_id, true);

    // Restaurar la ventana si está minimizada
    let _ = ShowWindow(hwnd, SW_RESTORE);

    // Intentar colocar la ventana en primer plano
    let result = SetForegroundWindow(hwnd).as_bool();

    if !result {
        println!("Intentando BringWindowToTop...");
        let _ = BringWindowToTop(hwnd);
        println!("Intentando SwitchToThisWindow...");
        let _ = SwitchToThisWindow(hwnd, true);
    }

    // Desconectar los hilos de entrada
    let _ = AttachThreadInput(foreground_thread_id, current_thread_id, false);

    if !result {
        println!("Forzando la ventana al frente con SetWindowPos...");
        SetWindowPos(
            hwnd,
            HWND_TOPMOST,
            0,
            0,
            0,
            0,
            SWP_NOMOVE | SWP_NOSIZE | SWP_SHOWWINDOW,
        )
            .is_ok() // Verifica si SetWindowPos fue exitoso
    } else {
        result
    }
}
