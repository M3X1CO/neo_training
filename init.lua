-- ==========================
-- BASIC VIM OPTIONS
-- ==========================
vim.g.mapleader = " "

-- Indentation settings (global defaults)
vim.opt.tabstop = 2
vim.opt.shiftwidth = 2
vim.opt.expandtab = true
vim.opt.autoindent = true
vim.opt.smartindent = true

-- Line wrapping and visual guides
vim.opt.textwidth = 0
vim.opt.wrap = true
vim.opt.linebreak = true
vim.opt.colorcolumn = "70"
vim.opt.formatoptions = "croql"

-- Better editing experience
vim.opt.number = true -- Show line numbers
vim.opt.relativenumber = true -- Relative line numbers
vim.opt.cursorline = true -- Highlight current line
vim.opt.signcolumn = "yes" -- Always show sign column
vim.opt.updatetime = 250 -- Faster completion

-- Search settings
vim.opt.ignorecase = true -- Ignore case in search
vim.opt.smartcase = true -- Unless capital letter in search

-- ==========================
-- BOOTSTRAP LAZY.NVIM
-- ==========================
vim.opt.rtp:prepend("~/.local/share/nvim/lazy/lazy.nvim")

-- ==========================
-- PLUGINS
-- ==========================
require("lazy").setup({
  {
    "nvim-treesitter/nvim-treesitter",
    build = ":TSUpdate",
  },
  {
    "windwp/nvim-autopairs",
    event = "InsertEnter",
    config = function()
      require("nvim-autopairs").setup({
        check_ts = true, -- Use treesitter
        ts_config = {
          lua = { "string" }, -- Don't add pairs in lua string treesitter nodes
          javascript = { "template_string" },
          java = false, -- Don't check treesitter on java
        },
      })
    end,
  },

})

-- ==========================
-- LSP CONFIG (Modern Neovim 0.11+)
-- ==========================
-- Rust LSP
vim.lsp.config("rust-analyzer", {
  cmd = { "rust-analyzer" },
  root_markers = { "Cargo.toml", "rust-project.json" },
  settings = {
    ["rust-analyzer"] = {
      formatting = {
        enable = true,
      },
      checkOnSave = {
        command = "clippy",
      },
    },
  },
})

-- Lua LSP (optional but recommended)
vim.lsp.config("lua-language-server", {
  cmd = { "lua-language-server" },
  root_markers = { ".luarc.json", ".luarc.jsonc", ".luacheckrc", ".stylua.toml", "stylua.toml" },
  settings = {
    Lua = {
      diagnostics = {
        globals = { "vim" },
      },
      workspace = {
        library = vim.api.nvim_get_runtime_file("", true),
        checkThirdParty = false,
      },
      telemetry = {
        enable = false,
      },
    },
  },
})

-- Enable LSP for Rust files
vim.api.nvim_create_autocmd("FileType", {
  pattern = "rust",
  callback = function()
    vim.lsp.enable("rust-analyzer")
  end,
})

-- Enable LSP for Lua files
vim.api.nvim_create_autocmd("FileType", {
  pattern = "lua",
  callback = function()
    vim.lsp.enable("lua-language-server")
  end,
})

-- ==========================
-- TREESITTER CONFIG
-- ==========================
require("nvim-treesitter.configs").setup({
  ensure_installed = {
    "lua",
    "python",
    "javascript",
    "html",
    "css",
    "json",
    "rust",
    "java",
    "markdown",
  },
  auto_install = true,
  highlight = {
    enable = true,
    additional_vim_regex_highlighting = false,
  },
  indent = { enable = true },
})

-- ==========================
-- FILETYPE-SPECIFIC SETTINGS
-- ==========================
-- Rust: 4 spaces (Rust convention)
vim.api.nvim_create_autocmd("FileType", {
  pattern = "rust",
  callback = function()
    vim.opt_local.shiftwidth = 4
    vim.opt_local.tabstop = 4
    vim.opt_local.expandtab = true
  end,
})

-- Python: 4 spaces (PEP 8)
vim.api.nvim_create_autocmd("FileType", {
  pattern = "python",
  callback = function()
    vim.opt_local.shiftwidth = 4
    vim.opt_local.tabstop = 4
    vim.opt_local.expandtab = true
  end,
})

-- ==========================
-- MOVEMENT SWAPS (j/k swap)
-- ==========================
vim.keymap.set("n", "j", "k", { noremap = true, silent = true })
vim.keymap.set("n", "k", "j", { noremap = true, silent = true })
vim.keymap.set("v", "j", "k", { noremap = true, silent = true })
vim.keymap.set("v", "k", "j", { noremap = true, silent = true })
vim.keymap.set("o", "j", "k", { noremap = true, silent = true })
vim.keymap.set("o", "k", "j", { noremap = true, silent = true })

-- ==========================
-- JUMP TO MARK AND SCROLL TOP
-- ==========================
local function jump_to_mark(mark)
  return function()
    local ok = pcall(function()
      -- For global marks (A-Z), we need to handle buffer switching
      if mark:match("[A-Z]") then
        -- Save current position in case mark doesn't exist
        local pos = vim.api.nvim_win_get_cursor(0)
        vim.cmd("normal! `" .. mark)
        -- Check if we actually jumped (mark exists)
        local new_pos = vim.api.nvim_win_get_cursor(0)
        vim.cmd("normal! zt")
      else
        -- Local marks (a-z) work within current buffer
        vim.cmd("normal! `" .. mark)
        vim.cmd("normal! zt")
      end
    end)
    if not ok then
      vim.notify("Mark '" .. mark .. "' is not set", vim.log.levels.WARN)
    end
  end
end

-- Map lowercase marks (a-z) - local marks
for i = 97, 122 do
  local mark = string.char(i)
  vim.keymap.set("n", "'" .. mark, jump_to_mark(mark), { noremap = true })
  vim.keymap.set("n", "`" .. mark, jump_to_mark(mark), { noremap = true })
end

-- Map uppercase marks (A-Z) - global marks  
for i = 65, 90 do
  local mark = string.char(i)
  vim.keymap.set("n", "'" .. mark, jump_to_mark(mark), { noremap = true })
  vim.keymap.set("n", "`" .. mark, jump_to_mark(mark), { noremap = true })
end

-- ==========================
-- FORMATTING KEYBINDINGS
-- ==========================
-- Format Rust file with rustfmt
vim.keymap.set(
  "n",
  "<leader>fr",
  ":!rustfmt %<CR>:e<CR>",
  { noremap = true, silent = true, desc = "Format Rust file" }
)

-- Format Lua file with stylua
vim.keymap.set(
  "n",
  "<leader>fl",
  ":!~/.cargo/bin/stylua %<CR>:e<CR>",
  { noremap = true, silent = true, desc = "Format Lua file" }
)

-- LSP formatting (works with rust-analyzer)
vim.keymap.set(
  "n",
  "<leader>f",
  vim.lsp.buf.format,
  { noremap = true, silent = true, desc = "Format with LSP" }
)

-- ==========================
-- LSP KEYBINDINGS
-- ==========================
vim.api.nvim_create_autocmd("LspAttach", {
  callback = function(args)
    local opts = { buffer = args.buf }
    vim.keymap.set("n", "gd", vim.lsp.buf.definition, opts)
    vim.keymap.set("n", "K", vim.lsp.buf.hover, opts)
    vim.keymap.set("n", "<leader>rn", vim.lsp.buf.rename, opts)
    vim.keymap.set("n", "<leader>ca", vim.lsp.buf.code_action, opts)
    vim.keymap.set("n", "gr", vim.lsp.buf.references, opts)
  end,
})

-- ==========================
-- USEFUL ADDITIONAL KEYBINDINGS
-- ==========================
-- Quick save
vim.keymap.set("n", "<leader>w", ":w<CR>", { desc = "Save file" })

-- Quick quit
vim.keymap.set("n", "<leader>q", ":q<CR>", { desc = "Quit" })

-- Clear search highlight
vim.keymap.set("n", "<leader>h", ":nohlsearch<CR>", { desc = "Clear highlight" })

-- Move lines up/down in visual mode
vim.keymap.set("v", "J", ":m '>+1<CR>gv=gv", { desc = "Move line down" })
vim.keymap.set("v", "K", ":m '<-2<CR>gv=gv", { desc = "Move line up" })

-- Easy buffer navigation
vim.keymap.set("n", "<leader>j", ":bprev<CR>", { desc = "Previous buffer (up)" })
vim.keymap.set("n", "<leader>k", ":bnext<CR>", { desc = "Next buffer (down)" })
vim.keymap.set("n", "<leader>bd", ":bd<CR>", { desc = "Delete buffer" })
vim.keymap.set("n", "<leader>", ":e ", { desc = "Edit file" })
vim.keymap.set("n", "<leader>ls", ":ls<CR>", { desc = "List buffers" })

-- Keep cursor centered when scrolling
vim.keymap.set("n", "<C-d>", "<C-d>zz")
vim.keymap.set("n", "<C-u>", "<C-u>zz")

