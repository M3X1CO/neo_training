
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
    vim.cmd("normal! '" .. mark)  -- Jump to the mark
    vim.cmd("normal! zt")          -- Scroll to put cursor at top of window
  end
end

-- Map all lowercase marks (a-z)
for i = 97, 122 do  -- ASCII 97-122 = a-z
  local mark = string.char(i)
  vim.keymap.set('n', "'" .. mark, jump_to_mark(mark), { noremap = true })
end

-- Optional: also map uppercase marks (A-Z) if you use them
for i = 65, 90 do  -- ASCII 65-90 = A-Z
  local mark = string.char(i)
  vim.keymap.set('n', "'" .. mark, jump_to_mark(mark), { noremap = true })
end


