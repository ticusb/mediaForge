export default function ToolPanel({ selectedTool, hasFile }) {
    const getToolContent = () => {
        switch (selectedTool) {
            case "convert":
                return (
                    <div className="space-y-4">
                        <div>
                            <label className="block text-sm font-medium text-gray-700 mb-2">
                                Output Format
                            </label>
                            <select
                                disabled={!hasFile}
                                className="w-full px-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-blue-500 focus:border-transparent disabled:bg-gray-100 disabled:cursor-not-allowed"
                            >
                                <option>JPG</option>
                                <option>PNG</option>
                                <option>WEBP</option>
                                <option>GIF</option>
                            </select>
                        </div>

                        <div>
                            <label className="block text-sm font-medium text-gray-700 mb-2">
                                Quality
                            </label>
                            <input
                                type="range"
                                min="1"
                                max="100"
                                defaultValue="80"
                                disabled={!hasFile}
                                className="w-full disabled:opacity-50"
                            />
                            <div className="flex justify-between text-xs text-gray-500 mt-1">
                                <span>Low</span>
                                <span>High</span>
                            </div>
                        </div>

                        <div>
                            <label className="block text-sm font-medium text-gray-700 mb-2">
                                Resize
                            </label>
                            <div className="grid grid-cols-2 gap-2">
                                <input
                                    type="number"
                                    placeholder="Width"
                                    disabled={!hasFile}
                                    className="px-3 py-2 border border-gray-300 rounded-lg text-sm disabled:bg-gray-100"
                                />
                                <input
                                    type="number"
                                    placeholder="Height"
                                    disabled={!hasFile}
                                    className="px-3 py-2 border border-gray-300 rounded-lg text-sm disabled:bg-gray-100"
                                />
                            </div>
                        </div>
                    </div>
                );

            case "remove-bg":
                return (
                    <div className="space-y-4">
                        <div className="p-4 bg-blue-50 border border-blue-200 rounded-lg">
                            <p className="text-sm text-blue-800">
                                Upload an image to automatically remove its
                                background
                            </p>
                        </div>

                        <button
                            disabled={!hasFile}
                            className="w-full px-4 py-3 bg-blue-500 text-white rounded-lg font-medium hover:bg-blue-600 disabled:bg-gray-300 disabled:cursor-not-allowed transition-colors"
                        >
                            Remove Background
                        </button>

                        <div>
                            <label className="block text-sm font-medium text-gray-700 mb-2">
                                Replace Background
                            </label>
                            <input
                                type="color"
                                defaultValue="#ffffff"
                                disabled={!hasFile}
                                className="w-full h-12 rounded-lg cursor-pointer disabled:cursor-not-allowed"
                            />
                        </div>
                    </div>
                );

            case "color-grade":
                return (
                    <div className="space-y-4">
                        <div>
                            <label className="block text-sm font-medium text-gray-700 mb-3">
                                Presets
                            </label>
                            <div className="grid grid-cols-2 gap-2">
                                {[
                                    "Vintage",
                                    "Cinematic",
                                    "Bright",
                                    "Moody",
                                ].map((preset) => (
                                    <button
                                        key={preset}
                                        disabled={!hasFile}
                                        className="px-3 py-2 border-2 border-gray-300 rounded-lg text-sm font-medium hover:border-blue-500 hover:bg-blue-50 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
                                    >
                                        {preset}
                                    </button>
                                ))}
                            </div>
                        </div>

                        <div className="pt-2 border-t border-gray-200">
                            <label className="block text-sm font-medium text-gray-700 mb-2">
                                Brightness
                            </label>
                            <input
                                type="range"
                                min="-100"
                                max="100"
                                defaultValue="0"
                                disabled={!hasFile}
                                className="w-full disabled:opacity-50"
                            />
                        </div>

                        <div>
                            <label className="block text-sm font-medium text-gray-700 mb-2">
                                Contrast
                            </label>
                            <input
                                type="range"
                                min="-100"
                                max="100"
                                defaultValue="0"
                                disabled={!hasFile}
                                className="w-full disabled:opacity-50"
                            />
                        </div>

                        <div>
                            <label className="block text-sm font-medium text-gray-700 mb-2">
                                Saturation
                            </label>
                            <input
                                type="range"
                                min="-100"
                                max="100"
                                defaultValue="0"
                                disabled={!hasFile}
                                className="w-full disabled:opacity-50"
                            />
                        </div>
                    </div>
                );

            default:
                return null;
        }
    };

    return (
        <div className="bg-white rounded-xl border border-gray-200 p-6 shadow-sm sticky top-8">
            <div className="mb-6">
                <h3 className="text-lg font-semibold text-gray-800 mb-2">
                    Tool Settings
                </h3>
                <p className="text-sm text-gray-500">
                    {!hasFile
                        ? "Upload a file to get started"
                        : "Adjust settings below"}
                </p>
            </div>

            {getToolContent()}

            {!hasFile && (
                <div className="mt-6 p-4 bg-gray-50 border border-gray-200 rounded-lg text-center">
                    <div className="text-3xl mb-2">⚙️</div>
                    <p className="text-sm text-gray-600">
                        Tool options will appear here once you upload a file
                    </p>
                </div>
            )}
        </div>
    );
}
