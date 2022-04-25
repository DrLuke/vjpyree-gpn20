from PyreeEngine.layers import LayerContext

from pythonosc.dispatcher import Handler

from typing import Dict


class OscRandomVal:
    def __init__(self, index: int, context: LayerContext) -> None:
        self.val: float = 0.
        self.index: int = index
        self.context: LayerContext = context

        self.dispatcher_handler: Handler = context.oscdispatcher.map(f"/randomval/{index}", self.handler)

    def handler(self, addr, *args) -> None:
        if len(args) == 1 and type(args[0]) == float:
            self.val = args[0]

    def __call__(self, *args, **kwargs) -> Dict[str, float]:
        return {f"randomVal{self.index}": self.val}

    def __del__(self):
        self.context.oscdispatcher.unmap(f"/randomval/{self.index}", self.dispatcher_handler)


class OscToggle:
    def __init__(self, index: int, context: LayerContext) -> None:
        self.val: bool = False
        self.index: int = index
        self.context: LayerContext = context

        self.dispatcher_handler: Handler = context.oscdispatcher.map(f"/toggle/{index}", self.handler)

    def handler(self, addr, *args) -> None:
        if len(args) == 1 and type(args[0]) == float:
            self.val = bool(args[0])

    def __call__(self, *args, **kwargs) -> Dict[str, float]:
        return {f"toggle{self.index}": float(self.val)}

    def __del__(self):
        self.context.oscdispatcher.unmap(f"/toggle/{self.index}", self.dispatcher_handler)
