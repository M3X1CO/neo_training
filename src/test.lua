-- ==========================
-- MOVEMENT SWAPS
-- ==========================

-- Normal mode
vim.keymap.set('n', 'j', 'k', { noremap = true, silent = true })
vim.keymap.set('n', 'k', 'j', { noremap = true, silent = true })

-- Visual mode
vim.keymap.set('v', 'j', 'k', { noremap = true, silent = true })
vim.keymap.set('v', 'k', 'j', { noremap = true, silent = true })

-- Operator-pending mode
vim.keymap.set('o', 'j', 'k', { noremap = true, silent = true })
vim.keymap.set('o', 'k', 'j', { noremap = true, silent = true })


-- ==========================
-- JUMP TO MARK AND SCROLL TOP
-- ==========================
local function jump_to_mark(mark)
  return function()
    local ok = pcall(function()
      vim.cmd("normal! '" .. mark)
      vim.cmd("normal! zt")
    end)
    if not ok then
      vim.notify("Mark '" .. mark .. "' is not set", vim.log.levels.WARN)
    end
  end
end

-- Map all lowercase marks (a-z)
for i = 97, 122 do  -- ASCII 97-122 = a-z
  local mark = string.char(i)
  vim.keymap.set('n', "'" .. mark, jump_to_mark(mark), { noremap = true })
end

-- Optional: also map uppercase marks (A-Z) for Global Marks
for i = 65, 90 do  -- ASCII 65-90 = A-Z
  local mark = string.char(i)
  vim.keymap.set('n', "'" .. mark, jump_to_mark(mark), { noremap = true })
end

-- ==========================
-- LINE WIDTH AND COLOR COLUMN
-- ==========================
-- Set maximum text width to 70 for automatic formatting
vim.o.textwidth = 0

-- Soft wrap long lines visually (don't insert line breaks)
vim.o.wrap = true
vim.o.linebreak = true  -- Break at word boundaries

-- Highlight column 70 as a visual guide
vim.o.colorcolumn = "70"

-- Optional: automatically break lines at textwidth while typing
vim.o.formatoptions = "croql"

-- ==========================
-- MANUAL FORMATTING KEYBINDINGS
-- ==========================

-- Set leader key (if not already set)
vim.g.mapleader = " "

-- Format Rust file with rustfmt
vim.keymap.set('n', '<leader>fr', ':!rustfmt %<CR>:e<CR>', 
  { noremap = true, silent = true, desc = "Format Rust file" })

-- Format Lua file with stylua (using full path)
vim.keymap.set('n', '<leader>fl', ':!~/.cargo/bin/stylua %<CR>:e<CR>', 
  { noremap = true, silent = true, desc = "Format Lua file" })


