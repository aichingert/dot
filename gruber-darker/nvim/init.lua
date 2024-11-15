vim.g.mapleader = " "

local lazypath = vim.fn.stdpath "data" .. "/lazy/lazy.nvim"
if not vim.uv.fs_stat(lazypath) then
    vim.fn.system {
        "git",
        "clone",
        "--filter=blob:none",
        "https://github.com/folke/lazy.nvim.git",
        "--branch=stable",
        lazypath,
    }
end

vim.opt.rtp:prepend(lazypath)

require("lazy").setup({
    {
        "nvim-treesitter/nvim-treesitter",
        build = ":TSUpdate",
        config = function()
            local configs = require ("nvim-treesitter.configs")
            configs.setup({
                ensure_installed = { 
                    "rust", "zig", "c", "cpp", "go", "vim", "lua", "asm", "odin",
                },
                sync_install = false,
                highlight = { enable = true },
                indent = { enable = false }
            })
        end
    },
    {
        lazy = false,
        priority = 1000,
        "blazkowolf/gruber-darker.nvim",
        name = "gruber-darker",
        config = function()
            vim.cmd.colorscheme("gruber-darker")
        end
    },
    {
        'nvim-telescope/telescope.nvim', branch = '0.1.x',

        dependencies = {
            { 'nvim-lua/plenary.nvim' },
            { "nvim-telescope/telescope-fzf-native.nvim", build = "make" },
        },

        config = function()
            local data = assert(vim.fn.stdpath "data") --[[@as string]]

            require("telescope").setup {
                extensions = {
                    wrap_results = true,

                    fzf = {},
                },
            }

            pcall(require("telescope").load_extension, "fzf")

            local builtin = require('telescope.builtin')
            vim.keymap.set('n', '<leader>ff', builtin.find_files, {})
            vim.keymap.set('n', '<leader>fg', builtin.live_grep, {})
            vim.keymap.set('n', '<leader>fb', builtin.buffers, {})
            vim.keymap.set('n', '<leader>fh', builtin.help_tags, {})
        end,
    },

})

vim.treesitter.language.register("mor", { "mo" })
local parser_config = require "nvim-treesitter.parsers".get_parser_configs()
parser_config.mor = {
  install_info = {
    url = "github.com/veqox/tree-sitter-mor",
    branch = "main",
    files = {"src/parser.c"}, 
    generate_requires_npm = false, 
    requires_generate_from_grammar = false, 
  },
  filetype = "mo",
}

vim.filetype.add({
    pattern = {
        [".*%.mo"] = "mor",
    }
})

-- vim.cmd[[ colorscheme undead ]]
-- vim.cmd[[ au VimLeave * set guicursor=a:hor100 ]]
