return {
    "nvim-telescope/telescope.nvim", branch = "0.1.x",

    dependencies = {
        { "nvim-lua/plenary.nvim" },
        { "nvim-telescope/telescope-fzf-native.nvim", build = "make" },
    },
    keys = {
        { "<leader>ff", function() require("mylocal.functions").find_files() end, desc="" },
        { "<leader>fg", function() require("mylocal.functions").live_grep() end, desc="" },
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
    end
}
