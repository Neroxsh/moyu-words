import { useEffect, useState } from "react";
import { useBookStore, useSettingsStore } from "./store";
import Header from "./components/Layout/Header";
import Sidebar from "./components/Layout/Sidebar";
import UnitsTab from "./components/Units/UnitsTab";
import ArchiveTab from "./components/Archive/ArchiveTab";
import SettingsTab from "./components/Settings/SettingsTab";
import StudyModeTab from "./components/StudyMode/StudyModeTab";

type Tab = "units" | "archive" | "study" | "settings";

export default function App() {
  const [activeTab, setActiveTab] = useState<Tab>("units");
  const fetchBooks = useBookStore((s) => s.fetchBooks);
  const fetchSettings = useSettingsStore((s) => s.fetchSettings);

  useEffect(() => {
    fetchBooks();
    fetchSettings();
  }, []);

  const renderTab = () => {
    switch (activeTab) {
      case "units": return <UnitsTab />;
      case "archive": return <ArchiveTab />;
      case "study": return <StudyModeTab />;
      case "settings": return <SettingsTab />;
    }
  };

  return (
    <div className="flex flex-col h-full">
      <Header />
      <div className="flex flex-1 overflow-hidden">
        <Sidebar activeTab={activeTab} onTabChange={setActiveTab} />
        <main className="flex-1 overflow-auto p-6">
          {renderTab()}
        </main>
      </div>
    </div>
  );
}