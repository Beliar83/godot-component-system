import subprocess
import sys

def can_build(env, platform):
    try:
        process = subprocess.run("rustup toolchain list", capture_output=True, text=True, shell=True)
        process.check_returncode()
        return True
    except Exception as err:
        print("Rustup does not seem to be installed, or is not found in the current path. Please check "
              "https://rustup.rs/ for how to install rustup.")
        return False

def configure(env):
    print("Checking for installed rust toolchain")
    if sys.platform == 'win32':
        os = 'pc-windows'
    elif sys.platform == 'darwin':
        os = 'osx' #todo: check
    else:
        os = 'unknown-linux'
    try:
        process = subprocess.run("rustup toolchain list", capture_output=True, text=True, shell=True)
        process.check_returncode()
        lines = process.stdout.splitlines()
        channel = None
        for line in lines:
            if line.endswith('(default)'):
                if line.startswith('stable'):
                    channel = 'stable'
                    break
                elif line.startswith('nightly'):
                    channel = 'nightly'
                    break
        if channel is None:
            print('summator_rust: No toolchain detected. Installing stable toolchain')
            channel = 'stable'

        if env['platform'] == 'javascript':
            print('summator_rust: installing emscripen target for rust')
            process = subprocess.run('rustup target add wasm32-unknown-emscripten', shell=True)
            process.check_returncode()
        elif env.msvc:
            print('summator_rust: Installing and/or selecting rust msvc toolchain')
            process = subprocess.run('rustup target add x86_64-' + os + '-msvc', shell=True)
            process.check_returncode()
            process = subprocess.run('rustup default ' + channel + '-x86_64-' + os + '-msvc', shell=True)
            process.check_returncode()
        else:
            print('summator_rust: Installing and/or selecting rust gnu toolchain')
            process = subprocess.run('rustup target add x86_64-' + os + '-gnu', shell=True)
            process.check_returncode()
            process = subprocess.run('rustup default ' + channel + '-x86_64-' + os + '-gnu', shell=True)
            process.check_returncode()

    except Exception:
        raise RuntimeError("Rustup does not seem to be installed, or is not found in the current path. Please check "
                           "https://rustup.rs/ for how to install rustup.")
    pass
