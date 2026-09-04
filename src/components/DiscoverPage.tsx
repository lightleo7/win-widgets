import {
  Button,
  Checkbox,
  CheckboxGroup,
  Description,
  Input,
  Label,
} from "@heroui/react";

import {
  Download,
  Loader2,
  Plus,
  Sparkles,
} from "lucide-react";

import { SectionCard } from "./SectionCard";


export type WidgetCandidate = {
  id: string;
  name: string;
  description: string;
  author?: string | null;
  relativePath: string;
};


export type DownloadedPack = {
  token: string;
  author: string;
  pack: string;
  widgets: WidgetCandidate[];
};


type DiscoverPageProps = {
  widgetUrl: string;

  onWidgetUrlChange: (
    value: string
  ) => void;

  onInstall: () => void;

  isLoading: boolean;

  pack: DownloadedPack | null;

  selectedWidgets: string[];

  onSelectedWidgetsChange: (
    widgets: string[]
  ) => void;

  onInstallSelected: () => void;

  isInstalling: boolean;

  error: string | null;
};


export function DiscoverPage({
  widgetUrl,
  onWidgetUrlChange,
  onInstall,

  isLoading,

  pack,

  selectedWidgets,
  onSelectedWidgetsChange,

  onInstallSelected,

  isInstalling,

  error,
}: DiscoverPageProps) {

  return (
    <div className="max-w-[850px]">

      <SectionCard>

        <div className="px-6 py-6">

          <div className="flex items-start gap-4">

            <div
              className="
                flex
                h-11
                w-11
                shrink-0
                items-center
                justify-center
                rounded-xl
                bg-[#1b1c21]
                text-zinc-400
              "
            >
              <Download size={19} />
            </div>

            <div>

              <h2 className="font-semibold text-zinc-100">
                Install a widget
              </h2>

              <p className="mt-2 max-w-[600px] text-sm leading-6 text-zinc-500">
                Paste a link to a GitHub widget package.
                Win Widgets will scan the package and let you
                choose which widgets to install.
              </p>

            </div>

          </div>


          <div className="mt-7 flex gap-3">

            <Input
              value={widgetUrl}
              onChange={(event) =>
                onWidgetUrlChange(
                  event.target.value
                )
              }
              placeholder="https://github.com/author/widget-pack"
              disabled={isLoading || isInstalling}
              className="
                h-11
                flex-1
                rounded-xl
                border
                border-white/[0.08]
                bg-[#191a1f]
                px-4
                text-sm
                text-zinc-100
                shadow-none
                placeholder:text-zinc-600
                hover:border-white/[0.12]
                focus:border-blue-500/70
                focus:ring-2
                focus:ring-blue-500/10
              "
            />


            <Button
              onPress={onInstall}
              isDisabled={
                !widgetUrl.trim()
                || isLoading
                || isInstalling
              }
              className="
                h-11
                shrink-0
                rounded-xl
                bg-blue-500
                px-5
                font-medium
                text-white
                shadow-none
                hover:bg-blue-400
              "
            >

              {isLoading ? (
                <Loader2
                  size={17}
                  className="animate-spin"
                />
              ) : (
                <Plus size={17} />
              )}

              {isLoading
                ? "Downloading..."
                : "Install"
              }

            </Button>

          </div>


          {error && (

            <div
              className="
                mt-4
                rounded-xl
                border
                border-red-500/20
                bg-red-500/[0.06]
                px-4
                py-3
                text-sm
                text-red-400
              "
            >
              {error}
            </div>

          )}

        </div>

      </SectionCard>


      {pack && (

        <SectionCard>

          <div className="px-6 py-6">

            <div className="flex items-start justify-between gap-4">

              <div>

                <h2 className="font-semibold text-zinc-100">
                  {pack.pack}
                </h2>

                <p className="mt-1 text-sm text-zinc-500">
                  {pack.author}
                  {" · "}
                  {pack.widgets.length} widget
                  {pack.widgets.length !== 1 && "s"}
                </p>

              </div>

            </div>


            <CheckboxGroup
              value={selectedWidgets}
              onChange={onSelectedWidgetsChange}
              className="mt-6"
            >

              <Label>
                Select widgets to install
              </Label>

              <Description>
                Choose which widgets from this package
                you want to add to your library.
              </Description>


              <div className="mt-4 flex flex-col gap-2">

                {pack.widgets.map((widget) => (

                  <Checkbox
                    key={widget.id}
                    value={widget.relativePath}
                    className="
                      rounded-xl
                      border
                      border-white/[0.06]
                      bg-[#191a1f]
                      px-4
                      py-3
                      transition
                      hover:border-white/[0.12]
                    "
                  >

                    <Checkbox.Content>

                      <Checkbox.Control>
                        <Checkbox.Indicator />
                      </Checkbox.Control>

                      <div>

                        <div className="font-medium text-zinc-200">
                          {widget.name}
                        </div>

                        {widget.description && (

                          <Description>
                            {widget.description}
                          </Description>

                        )}

                        <div className="mt-1 font-mono text-[11px] text-zinc-600">
                          {widget.id}
                        </div>

                      </div>

                    </Checkbox.Content>

                  </Checkbox>

                ))}

              </div>

            </CheckboxGroup>


            <div className="mt-6 flex justify-end">

              <Button
                onPress={onInstallSelected}
                isDisabled={
                  selectedWidgets.length === 0
                  || isInstalling
                }
                className="
                  h-10
                  rounded-xl
                  bg-blue-500
                  px-5
                  font-medium
                  text-white
                  hover:bg-blue-400
                "
              >

                {isInstalling ? (
                  <Loader2
                    size={16}
                    className="animate-spin"
                  />
                ) : (
                  <Download size={16} />
                )}

                {isInstalling
                  ? "Installing..."
                  : `Install selected (${selectedWidgets.length})`
                }

              </Button>

            </div>

          </div>

        </SectionCard>

      )}


      {!pack && !isLoading && (

        <div
          className="
            mt-5
            rounded-3xl
            border
            border-dashed
            border-white/[0.08]
            bg-[#101115]
            px-6
            py-10
            text-center
          "
        >

          <Sparkles
            size={24}
            className="mx-auto text-zinc-600"
          />

          <h3 className="mt-4 font-medium text-zinc-300">
            Widget packages
          </h3>

          <p className="mx-auto mt-2 max-w-md text-sm leading-6 text-zinc-500">
            Paste a GitHub repository URL to browse
            the widgets inside the package.
          </p>

        </div>

      )}

    </div>
  );
}