local M = {}

M.setup = function()
    require("nvim-treesitter").setup {
	sync_install = false,
  	auto_install = true,

	highlight = {
	    enable = true,
	},
    }
end

M.setup()

return M
