export default function Sidebar({ selectedTool, onToolSelect }) {
    const tools = [
        {
            id: "convert",
            name: "Convert",
            icon: "🔄",
            description: "Change file formats",
        },
        {
            id: "remove-bg",
            name: "Remove Background",
            icon: "✂️",
            description: "Remove or replace backgrounds",
        },
        {
            id: "color-grade",
            name: "Color Grade",
            icon: "🎨",
            description: "Adjust colors and apply presets",
        },
    ];

    return (
        <aside className="w-64 bg-white border-r border-gray-200 flex flex-col">
            {/* Logo */}
            <div className="p-6 border-b border-gray-200">
                <h1 className="text-2xl font-bold text-gray-800 flex items-center gap-2">
                    <span>🎬</span>
                    <span>MediaForge</span>
                </h1>
            </div>

            {/* Tool List */}
            <nav className="flex-1 p-4">
                <div className="space-y-2">
                    {tools.map((tool) => (
                        <button
                            key={tool.id}
                            onClick={() => onToolSelect(tool.id)}
                            className={`w-full text-left px-4 py-3 rounded-lg transition-all ${
                                selectedTool === tool.id
                                    ? "bg-blue-50 border-2 border-blue-500 text-blue-700"
                                    : "bg-white border-2 border-transparent text-gray-700 hover:bg-gray-50 hover:border-gray-200"
                            }`}
                        >
                            <div className="flex items-start gap-3">
                                <span className="text-2xl">{tool.icon}</span>
                                <div className="flex-1">
                                    <div className="font-semibold">
                                        {tool.name}
                                    </div>
                                    <div className="text-xs text-gray-500 mt-0.5">
                                        {tool.description}
                                    </div>
                                </div>
                            </div>
                        </button>
                    ))}
                </div>
            </nav>

            {/* Footer */}
            <div className="p-4 border-t border-gray-200">
                <div className="text-xs text-gray-500 text-center">
                    MediaForge MVP v0.1.0
                </div>
            </div>
        </aside>
    );
}
