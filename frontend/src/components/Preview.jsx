import { useState, useEffect } from "react";

export default function Preview({ file }) {
    const [previewUrl, setPreviewUrl] = useState(null);

    useEffect(() => {
        if (!file) {
            setPreviewUrl(null);
            return;
        }

        const url = URL.createObjectURL(file);
        setPreviewUrl(url);

        return () => {
            URL.revokeObjectURL(url);
        };
    }, [file]);

    if (!file) return null;

    const isImage = file.type.startsWith("image/");
    const isVideo = file.type.startsWith("video/");
    const fileSize = (file.size / 1024).toFixed(2);
    const fileSizeUnit = file.size > 1024 * 1024 ? "MB" : "KB";
    const displaySize =
        file.size > 1024 * 1024
            ? (file.size / (1024 * 1024)).toFixed(2)
            : fileSize;

    return (
        <div className="bg-white rounded-xl border border-gray-200 overflow-hidden shadow-sm">
            {/* Header */}
            <div className="px-6 py-4 border-b border-gray-200 bg-gray-50">
                <h3 className="text-lg font-semibold text-gray-800">Preview</h3>
            </div>

            {/* File info */}
            <div className="px-6 py-4 bg-gray-50 border-b border-gray-200">
                <div className="grid grid-cols-2 gap-4 text-sm">
                    <div>
                        <span className="text-gray-500">Filename:</span>
                        <div className="font-medium text-gray-800 truncate mt-1">
                            {file.name}
                        </div>
                    </div>
                    <div>
                        <span className="text-gray-500">Size:</span>
                        <div className="font-medium text-gray-800 mt-1">
                            {displaySize} {fileSizeUnit}
                        </div>
                    </div>
                    <div>
                        <span className="text-gray-500">Type:</span>
                        <div className="font-medium text-gray-800 mt-1">
                            {file.type || "Unknown"}
                        </div>
                    </div>
                    <div>
                        <span className="text-gray-500">Status:</span>
                        <div className="font-medium text-green-600 mt-1">
                            ✓ Ready to process
                        </div>
                    </div>
                </div>
            </div>

            {/* Preview area */}
            <div className="p-6 bg-gray-900 min-h-[400px] flex items-center justify-center">
                {isImage && (
                    <img
                        src={previewUrl}
                        alt={file.name}
                        className="max-w-full max-h-[400px] object-contain rounded-lg"
                    />
                )}

                {isVideo && (
                    <video
                        src={previewUrl}
                        controls
                        className="max-w-full max-h-[400px] rounded-lg"
                    >
                        Your browser does not support video playback.
                    </video>
                )}

                {!isImage && !isVideo && (
                    <div className="text-center text-gray-400">
                        <div className="text-5xl mb-4">📄</div>
                        <div>Preview not available for this file type</div>
                    </div>
                )}
            </div>
        </div>
    );
}
