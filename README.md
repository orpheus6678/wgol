# wgol: game of life, in webassembly

a rewrite of conways game of life harnessing the blazing fast speeds of rust powered webassembly.
it lags profusely when relying on the `web_sys::console::log_*` family of functions.
which is why logging functionality is conditionally compiled for dev builds only, using the `debug_assertions` attribute.
