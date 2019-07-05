# prelude

def bytestring(byte_list):
    return bytes(byte_list)

def domstring(byte_list):
    return bytes(byte_list).decode('utf_8', 'strict')

def bind_exports(instance, builders=None):
    class Exports:
        def __init__(self, export_builders):
            self.exports = {name: builder(instance) for name, builder in export_builders.items()}

        def __getattr__(self, export_name):
            return self.exports[export_name]

    return Exports(builders or export_builders)

# !prelude
