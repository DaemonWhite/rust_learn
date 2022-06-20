project('rust_learn', 'rust',
          version: '0.1.0',
    meson_version: '>= 0.56.0',
  default_options: [ 'warning_level=2',
                   ],
)

i18n = import('i18n')
gnome = import('gnome')


dependency('gtk4', version: '>= 4.4')


cargo_sources = files(
  'Cargo.toml',
  'Cargo.lock',
  'build.rs'
)

subdir('src')

