local opt = vim.opt

opt.inccommand = "split"

opt.smartcase = true
opt.ignorecase = true

opt.number = true
opt.relativenumber = true

opt.tabstop = 4
opt.softtabstop = 4
opt.shiftwidth = 4
opt.expandtab = true
opt.termguicolors = true
opt.scrolloff = 15

opt.formatoptions:remove "o"
