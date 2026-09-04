import { Button } from "@heroui/react";
import {
  RefreshCw,
  Tv,
} from "lucide-react";

type AppHeaderProps = {
  onReload: () => void;
};

export function AppHeader({
  onReload,
}: AppHeaderProps) {
  return (
    <header className="border-b border-white/[0.06]">
      <div className="mx-auto flex max-w-[1400px] items-center justify-between px-8 py-7">

        <div className="flex items-center gap-4">

          <div
            className="
              flex
              h-12
              w-12
              items-center
              justify-center
              rounded-2xl
              border
              border-white/[0.08]
              bg-[#17181d]
            "
          >
            <Tv
              size={23}
              strokeWidth={1.8}
              className="text-zinc-200"
            />
          </div>

          <div>
            <h1 className="text-xl font-semibold tracking-tight text-zinc-100">
              Win Widgets
            </h1>

            <p className="mt-1 text-sm text-zinc-500">
              Desktop widget manager
            </p>
          </div>

        </div>

        <Button
          onPress={onReload}
          className="
            h-10
            rounded-xl
            bg-blue-500
            px-5
            font-medium
            text-white
            shadow-none
            hover:bg-blue-400
          "
        >
          <RefreshCw size={17} />

          Reload widgets
        </Button>

      </div>
    </header>
  );
}