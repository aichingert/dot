return {
    {
        "nvim-treesitter/nvim-treesitter",
        dependencies = {},
        build = ":TSUpdate",
        config = function()
            require("custom.treesitter").setup()
        end,
    },
}
