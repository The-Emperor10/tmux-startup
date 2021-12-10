pkgname=tmux-startup
pkgver=0.1.0
pkgrel=1
depends=('tmux' 'glibc')
makedepends=('rust' 'cargo')
arch=('i686' 'x86_64' 'armv6h' 'armv7h')

build() {
    cargo build --release
}

package() {
    #install -d "${pkgdir}/usr/bin/"
    #install -d "${pkgdir}/usr/lib/systemd/system/"
    #install -d "${pkgdir}/usr/share/bash-completion/completions"
    install -Dm755 "${startdir}/target/release/tmux-startup" "${pkgdir}/usr/bin/tmux-startup"
    install -Dm644 "${startdir}/tmux-startup@.service" "${pkgdir}/usr/lib/systemd/system/tmux-startup@.service"
    install -Dm644 "${startdir}/tmux-startup.bash" "${pkgdir}/usr/share/bash-completion/completions/tmux-startup"
    install -Dm644 "${startdir}/tmux-startup@.service" "${pkgdir}/usr/share/zsh/functions/Completion/Unix/_tmux-startup"
}
