
local colors = {
    handmade_beige = "#cdaa7d",
    handmade_dark_blue = "#191970",
    handmade_dark_gray = "#161616",
    handmade_dark_green = "#006400",
    handmade_gold = "#cd950c",
    handmade_light_beige = "#dab98f",
    handmade_light_gray = "#7f7f7f",
    handmade_light_green = "#40ff40",
    handmade_olive = "#6b8e23",
    handmade_red = "#ff0000",
    handmade_yellow = "#ffff00",
    handmade_green = "#79815e",
}

local hl = vim.api.nvim_set_hl
local theme = {}

theme.set_highlights = function()
    hl(0, "Normal", { fg = colors.handmade_beige, bg = 'NONE' })
    hl(0, "SignColumn", { fg = 'NONE', bg = colors.handmade_dark_gray })
    hl(0, "MsgArea", { fg = colors.handmade_beige, bg = colors.handmade_dark_gray })
    hl(0, "ModeMsg", { fg = colors.handmade_beige, bg = colors.handmade_dark_gray })
    hl(0, "MsgSeparator", { fg = colors.handmade_beige, bg = colors.handmade_dark_gray })
    hl(0, "SpellBad", { fg = colors.handmade_red, bg = 'NONE', underline = true, })
    hl(0, "SpellCap", { fg = colors.handmade_gold, bg = 'NONE', underline = true, })
    hl(0, "SpellLocal", { fg = colors.handmade_dark_green, bg = 'NONE', underline = true, })
    hl(0, "NormalNC", { fg = colors.handmade_beige, bg = colors.handmade_dark_gray })
    hl(0, "Pmenu", { fg = colors.handmade_light_gray, bg = colors.handmade_dark_gray, sp = 'NONE', blend = 50, })
    hl(0, "WildMenu", { fg = colors.handmade_beige, bg = colors.handmade_dark_blue })
    hl(0, "CursorLineNr", { fg = colors.handmade_beige, bg = 'NONE', bold = true, })
    hl(0, "Comment", { fg = colors.handmade_light_gray, bg = 'NONE', italic = true, })
    hl(0, "LineNr", { fg = colors.handmade_light_grayred, bg = 'NONE' })
    hl(0, "FloatBorder", { fg = colors.handmade_light_beige, bg = colors.handmade_dark_gray })
    hl(0, "Whitespace", { fg = colors.handmade_dark_gray, bg = 'NONE' })
    hl(0, "VertSplit", { fg = colors.handmade_beige, bg = colors.handmade_dark_gray })
    hl(0, "CursorLine", { fg = 'NONE', bg = colors.handmade_light_gray })
    hl(0, "CursorColumn", { fg = 'NONE', bg = colors.handmade_light_gray })
    hl(0, "ColorColumn", { fg = 'NONE', bg = colors.handmade_light_gray })
    hl(0, "NormalFloat", { fg = 'NONE', bg = colors.handmade_light_gray })
    hl(0, "WarningMsg", { fg = colors.handmade_red, bg = colors.handmade_dark_gray })
    hl(0, "MatchWord", { fg = 'NONE', bg = 'NONE', underline = true, })

    hl(0, "Cursor", { fg = colors.handmade_light_green, bg = colors.handmade_light_green })
    hl(0, "Cursor", { fg = colors.handmade_light_green, bg = 'NONE' })
    hl(0, "lCursor", { fg = colors.handmade_light_green, bg = 'NONE' })
    hl(0, "CursorIM", { fg = colors.handmade_light_green, bg = 'NONE' })
    hl(0, "TermCursor", { fg = colors.handmade_light_green, bg = 'NONE' })
    hl(0, "TermCursorNC", { fg = colors.handmade_light_green, bg = 'NONE' })
    hl(0, "Search", { fg = colors.handmade_beige, bg = colors.handmade_dark_gray })

    hl(0, "EndOfBuffer", { fg = colors.handmade_dark_gray, bg = 'NONE' })
    hl(0, "NonText", { fg = colors.handmade_dark_gray, bg = 'NONE' })
    hl(0, "Variable", { fg = colors.handmade_beige, bg = 'NONE' })
    hl(0, "@variable", { fg = colors.handmade_beige, bg = 'NONE' })
    hl(0, "String", { fg = colors.handmade_green, bg = 'NONE' })
    hl(0, "Character", { fg = colors.handmade_olive, bg = 'NONE' })
    hl(0, "Constant", { fg = colors.handmade_green, bg = 'NONE' })
    hl(0, "Number", { fg = colors.handmade_light_beige, bg = 'NONE' })
    hl(0, "Boolean", { fg = colors.handmade_beige, bg = 'NONE' })
    hl(0, "Float", { fg = colors.handmade_light_beige, bg = 'NONE' })
    hl(0, "Identifier", { fg = colors.handmade_light_beige, bg = 'NONE'})
    hl(0, "Function", { fg = colors.handmade_light_beige, bg = 'NONE' })
    hl(0, "Operator", { fg = colors.handmade_gold, bg = 'NONE' })
    hl(0, "Type", { fg = colors.handmade_light_beige, bg = 'NONE' })
    hl(0, "Structure", { fg = colors.handmade_light_beige, bg = 'NONE' })
    hl(0, "Typedef", { fg = colors.handmade_gold, bg = 'NONE' })
    hl(0, "Keyword", { fg = colors.handmade_gold, bg = 'NONE' })
    hl(0, "Statement", { fg = colors.handmade_light_beige, bg = 'NONE' })
    hl(0, "Conditional", { fg = colors.handmade_beige, bg = 'NONE' })
    hl(0, "Repeat", { fg = colors.handmade_beige, bg = 'NONE' })
    hl(0, "Label", { fg = colors.handmade_beige, bg = 'NONE' })
    hl(0, "Exception", { fg = colors.handmade_light_beige, bg = 'NONE' })
    hl(0, "Include", { fg = colors.handmade_light_beige, bg = 'NONE' })
    hl(0, "PreProc", { fg = colors.handmade_light_beige, bg = 'NONE' })
    hl(0, "Define", { fg = colors.handmade_gold, bg = 'NONE' })
    hl(0, "Macro", { fg = colors.handmade_gold, bg = 'NONE' })
    hl(0, "PreCondit", { fg = colors.handmade_light_beige, bg = 'NONE' })
    hl(0, "Special", { fg = colors.handmade_beige, bg = 'NONE' })
    hl(0, "SpecialChar", { fg = colors.handmade_gold, bg = 'NONE' })
    hl(0, "Tag", { fg = colors.handmade_light_beige, bg = 'NONE' })
    hl(0, "Debug", { fg = colors.handmade_red, bg = 'NONE' })
    hl(0, "Delimiter", { fg = colors.handmade_light_beige, bg = 'NONE' })
    hl(0, "SpecialComment", { fg = colors.handmade_light_beige, bg = 'NONE' })
    hl(0, "Underlined", { fg = 'NONE', bg = 'NONE', underline = true, })
    hl(0, "Bold", { fg = 'NONE', bg = 'NONE', bold = true, })
    hl(0, "Italic", { fg = 'NONE', bg = 'NONE', italic = true, })
    hl(0, "Ignore", { fg = colors.handmade_beige, bg = colors.handmade_dark_gray, bold = true, })
    hl(0, "Todo", { fg = colors.handmade_red, bg = colors.handmade_dark_gray, bold = true, })
    hl(0, "Error", { fg = colors.handmade_red, bg = colors.handmade_dark_gray, bold = true, })
    hl(0, "TabLine", { fg = colors.handmade_light_gray, bg = 'NONE'})
    hl(0, "TabLineSel", { fg = colors.handmade_beige, bg = 'NONE' })
    hl(0, "TabLineFill", { fg = colors.handmade_beige, bg = 'NONE' })
end

local M = {}

M.setup = function()
    vim.cmd("hi clear")

    vim.o.background = "dark"
    if vim.fn.exists("syntax_on") then
        vim.cmd("syntax reset")
    end

    vim.o.termguicolors = true
    vim.g.colors_name = "handmade"

    theme.set_highlights()
end

M.setup()
