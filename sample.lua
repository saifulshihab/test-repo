local function greet(name)
    local cleaned = name:match("^%s*(.-)%s*$")
    if cleaned == "" then
        return nil, "name must not be empty"
    end
    return "Hello, " .. cleaned .. "!"
end

local name = arg[1] or "world"
local message, err = greet(name)
if not message then
    io.stderr:write(err .. "\n")
    os.exit(1)
end

print(message)
