import { useState } from "react";

export default function UploadZone({ onFileSelect, disabled }) {
    const [isDragging, setIsDragging] = useState(false);

    const handleDragOver = (e) => {
        e.preventDefault();
        if (!disabled) {
            setIsDragging(true);
        }
    };

    const handleDragLeave = (e) => {
        e.preventDefault();
        setIsDragging(false);
    };

    const handleDrop = (e) => {
        e.preventDefault();
        setIsDragging(false);

        if (disabled) return;

        const files = e.dataTransfer.files;
        if (files && files[0]) {
            onFileSelect(files[0]);
        }
    };

    const handleFileChange = (e) => {
        if (disabled) return;

        const files = e.target.files;
        if (files && files[0]) {
            onFileSelect(files[0]);
        }
    };

    return (
        <div
            onDragOver={handleDragOver}
            onDragLeave={handleDragLeave}
            onDrop={handleDrop}
            className={`relative border-2 border-dashed rounded-xl p-16 text-center transition-all ${
                isDragging
                    ? "border-blue-500 bg-blue-50 scale-[1.02]"
                    : disabled
                    ? "border-gray-200 bg-gray-50 opacity-50 cursor-not-allowed"
                    : "border-gray-300 bg-white hover:border-blue-400 hover:bg-gray-50 cursor-pointer"
            }`}
        >
            {disabled && (
                <div className="absolute inset-0 flex items-center justify-center bg-white bg-opacity-90 rounded-xl">
                    <div className="text-center">
                        <div className="text-4xl mb-2">🔒</div>
                        <div className="font-semibold text-gray-700">
                            Free limit reached
                        </div>
                        <div className="text-sm text-gray-500 mt-1">
                            Upgrade to continue
                        </div>
                    </div>
                </div>
            )}

            <div className="space-y-4">
                <div className="text-6xl">📁</div>

                <div>
                    <h3 className="text-2xl font-semibold text-gray-800 mb-2">
                        {isDragging
                            ? "Drop your file here"
                            : "Drag & drop your file"}
                    </h3>
                    <p className="text-gray-500">
                        or click to browse from your computer
                    </p>
                </div>

                <div className="pt-4">
                    <input
                        type="file"
                        id="file-input"
                        onChange={handleFileChange}
                        disabled={disabled}
                        className="hidden"
                        accept="image/*,video/*"
                    />
                    <label htmlFor="file-input">
                        <button
                            type="button"
                            onClick={() =>
                                document.getElementById("file-input").click()
                            }
                            disabled={disabled}
                            className={`px-8 py-3 rounded-lg font-semibold transition-all ${
                                disabled
                                    ? "bg-gray-300 text-gray-500 cursor-not-allowed"
                                    : "bg-blue-500 text-white hover:bg-blue-600 shadow-md hover:shadow-lg"
                            }`}
                        >
                            Browse Files
                        </button>
                    </label>
                </div>

                <div className="pt-4 text-sm text-gray-400">
                    Supported formats: JPG, PNG, WEBP, GIF, HEIC, MP4, MOV, AVI
                </div>
            </div>
        </div>
    );
}
