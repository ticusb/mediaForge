import { useState } from "react";
import Sidebar from "./components/Sidebar";
import Header from "./components/Header";
import Workspace from "./components/Workspace";
import AuthModal from "./components/AuthModal";
import FreeLimitModal from "./components/FreeLimitModal";

export default function App() {
    // User state
    const [user, setUser] = useState(null);
    const [freeUploadsRemaining, setFreeUploadsRemaining] = useState(3);

    // File and job state
    const [file, setFile] = useState(null);
    const [jobs, setJobs] = useState([]);

    // Tool state
    const [selectedTool, setSelectedTool] = useState("convert");

    // Modal state
    const [showAuthModal, setShowAuthModal] = useState(false);
    const [showLimitModal, setShowLimitModal] = useState(false);

    const isPro = user?.isPro || false;
    const isFreeLimitReached = !isPro && freeUploadsRemaining === 0;

    const handleFileSelect = (selectedFile) => {
        if (isFreeLimitReached) {
            setShowLimitModal(true);
            return;
        }

        if (!isPro && freeUploadsRemaining === 1) {
            setShowLimitModal(true);
        }

        setFile(selectedFile);
    };

    const handleUpload = async () => {
        if (!file) {
            alert("Please select a file first");
            return;
        }

        if (isFreeLimitReached) {
            setShowLimitModal(true);
            return;
        }

        const formData = new FormData();
        formData.append("file", file, file.name);

        try {
            const response = await fetch("/api/upload", {
                method: "POST",
                body: formData,
            });

            if (!response.ok) {
                throw new Error("Upload failed");
            }

            const data = await response.json();

            setJobs((prevJobs) => [
                {
                    id: data.asset_id,
                    status: "uploaded",
                    filename: file.name,
                    watermarked: !isPro,
                    createdAt: new Date().toISOString(),
                },
                ...prevJobs,
            ]);

            if (!isPro) {
                const newRemaining = freeUploadsRemaining - 1;
                setFreeUploadsRemaining(newRemaining);
                if (newRemaining === 0) {
                    setShowLimitModal(true);
                }
            }

            setFile(null);
            alert(`Upload successful! Asset ID: ${data.asset_id}`);
        } catch (error) {
            console.error("Upload error:", error);
            alert(`Upload failed: ${error.message}`);
        }
    };

    const handleAuth = (userData) => {
        setUser(userData);
        if (userData.isPro) {
            setFreeUploadsRemaining(Infinity);
        }
        setShowAuthModal(false);
    };

    const handleUpgradeClick = () => {
        setShowAuthModal(true);
    };

    return (
        <div className="flex h-screen bg-gray-50">
            {/* Left Sidebar */}
            <Sidebar
                selectedTool={selectedTool}
                onToolSelect={setSelectedTool}
            />

            {/* Main Content Area */}
            <div className="flex-1 flex flex-col overflow-hidden">
                {/* Header */}
                <Header
                    user={user}
                    isPro={isPro}
                    freeUploadsRemaining={freeUploadsRemaining}
                    onUpgradeClick={handleUpgradeClick}
                />

                {/* Workspace */}
                <Workspace
                    file={file}
                    jobs={jobs}
                    selectedTool={selectedTool}
                    isFreeLimitReached={isFreeLimitReached}
                    onFileSelect={handleFileSelect}
                    onUpload={handleUpload}
                />
            </div>

            {/* Modals */}
            <AuthModal
                isOpen={showAuthModal}
                onClose={() => setShowAuthModal(false)}
                onAuth={handleAuth}
            />

            <FreeLimitModal
                isOpen={showLimitModal}
                onClose={() => setShowLimitModal(false)}
                onUpgrade={() => {
                    setShowLimitModal(false);
                    setShowAuthModal(true);
                }}
                remaining={freeUploadsRemaining}
            />
        </div>
    );
}
