return {
    "nvim-treesitter/nvim-treesitter",
    opts = {
        ensure_installed = { 
            "rust", "zig", "c", "cpp", "lua", "asm", "go", "javascript", "mor",
        },
        sync_install = false,
        highlight = { enable = true },
        indent = { enable = false }
    },
    build = ":TSUpdate",
    config = function(_, opts)
        require("nvim-treesitter.parsers").get_parser_configs().mor = {
            install_info = {
                url = "github.com/veqox/tree-sitter-mor",
                files = { "src/parser.c" },
                branch = "main"
            },
            filetype = "mo"
        }

        require("nvim-treesitter.configs").setup(opts)
    end
}
