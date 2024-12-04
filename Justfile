
default:
    just --list

@nu day:
    nu ./nu/runner.nu {{day}}

@rust day:
    cargo run --quiet -- {{day}}

@run day:
    just rust {{day}}
    just nu {{day}}
