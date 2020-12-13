(lang dune 2.7)
(name comodoro)
(license MIT)
(authors "soywod <clement.douin@posteo.net>")
(maintainers "soywod <clement.douin@posteo.net>")
(source (github soywod/comodoro))
(homepage "https://github.com/soywod/comodoro")

(generate_opam_files true)

(package
 (name comodoro)
 (synopsis "Pomodoro timer CLI")
 (depends
  (ocaml (>= 4.11.1))
  (dune (>= 2.7.1))
  base
  stdio
  toml
 ))
