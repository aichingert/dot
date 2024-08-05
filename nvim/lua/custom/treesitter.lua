local M = {}

M.setup = function()
    local group = vim.api.nvim_create_augroup("custom-treesitter", { clear = true })

    require("nvim-treesitter").setup {
        ensure_install = { "rust", "zig", "community" },
    }

    vim.api.nvim_create_autocmd("FileType", {
        group = group,
        callback = function(args)
            local bufnr = args.buf
            local ft = vim.bo[bufnr].filetype
            pcall(vim.treesitter.start)
        end
    })
end

M.setup()

return M
