local function start_client()
	vim.lsp.start({
		name = "cargotomllsp",
		cmd = { "cargotomllsp" },
		root_dir = vim.fs.dirname(vim.fs.find({ 'Cargo.toml' }, { upward = true })[1]),
	})
end

local function stop_client()
	vim.lsp.stop_client(vim.lsp.get_clients())
end

local function restart_client()
	stop_client()
	start_client()
end

start_client()

vim.api.nvim_create_autocmd('LspAttach', {
	callback = function(args)
		-- print(vim.inspect(args))
		-- vim.keymap.set('n', 'K', vim.lsp.buf.hover, { buffer = args.buf })
		vim.keymap.set('n', '<C-k>', restart_client, {})
	end,
})
