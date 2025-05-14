require("config.options")
require("config.keymaps")

require("config.lazy")
require("config.colorscheme")

-- Hack needed to keep terminal cursor
vim.cmd [[ set guicursor= ]]
--vim.api.nvim_create_autocmd("ExitPre", {
--	group = vim.api.nvim_create_augroup("Exit", { clear = true }),
--	command = "set guicursor=a:ver90",
--	desc = "Set cursor back to beam when leaving Neovim."
--})

