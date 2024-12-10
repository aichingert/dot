return {
    "veqox/tree-sitter-mor.nvim",
    dependencies = {
        "nvim-treesitter/nvim-treesitter",
    },
    config = function()
        require("tree-sitter-mor").setup()
    end,
    build = function()
        require("tree-sitter-mor").build()
    end
}
