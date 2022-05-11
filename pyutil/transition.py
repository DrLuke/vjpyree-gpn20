"""Animation helper classes"""

import math
from typing import Any, List
from types import FunctionType

import asyncio


def smoothstep(edge0: float, edge1: float, x: float) -> float:
    t: float = max(min((x - edge0) / (edge1 - edge0), 0.0, 1.0, 1.0), 0.0)
    return t * t * (3.0 - 2.0 * t)


def sign(x: float) -> float:
    if x >= 0:
        return 1
    else:
        return -1


class Transition():
    def __init__(self) -> None:
        self.curval: Any = 0
        self.targetval: Any = None

    def tick(self, dt: float) -> None:
        self.curval = self.targetval

    def __call__(self, *args, **kwargs) -> float:
        return self.curval

    def settarget(self, target: float) -> None:
        self.targetval = target

    def hardset(self, value: float) -> None:
        self.curval = value
        self.targetval = value

    def finished(self) -> bool:
        return self.curval == self.targetval

    async def await_target_reached(self) -> None:
        while self.curval != self.targetval:
            await asyncio.sleep(0.016)
        return


class InstantTransition(Transition):
    def settarget(self, target: float) -> None:
        self.targetval = target
        self.curval = self.targetval

    def __call__(self, *args, **kwargs) -> float:
        self.curval = self.targetval
        return self.curval

    def finished(self) -> bool:
        return True


class LinearTransition(Transition):
    """Linearly changes value to set target with set speed"""

    def __init__(self, speed: float = 1.):
        super(LinearTransition, self).__init__()
        self.speed: float = speed
        self.prevspeed: float = None
        self.resetspeed: bool = True
        self.curval: float = 0.
        self.targetval: float = 0.

    def tick(self, dt: float) -> None:
        diff = self.targetval - self.curval
        if diff < 0.:
            self.curval += max(diff, -self.speed * dt)
        elif diff > 0.:
            self.curval += min(diff, self.speed * dt)
        if self.resetspeed and self.prevspeed is not None and self.curval == self.targetval:
            self.speed = self.prevspeed

    def finished(self) -> bool:
        return self.curval == self.targetval

    def settargetintime(self, targetval: float, time: float = 1, resetspeed: bool = True):
        """Moves to targetval within set time"""
        self.targetval = targetval
        self.prevspeed = self.speed
        self.resetspeed = resetspeed
        self.speed = (self.targetval - self.curval) / time


class SmoothstepAnim(LinearTransition):
    """Just like LinearTransition, but applying smoothstep between whole numbers"""

    def __init__(self, speed: float = 1.) -> None:
        super(SmoothstepAnim, self).__init__()
        self.speed = speed
        self.curval: float = 0.
        self.targetval: float = 0.

    def tick(self, dt: float) -> None:
        diff: float = self.targetval - self.curval
        if diff < 0.:
            self.curval += max(diff, -self.speed) * dt
        elif diff > 0.:
            self.curval += min(diff, self.speed) * dt

    def __call__(self, *args, **kwargs) -> float:
        return math.floor(self.curval) + smoothstep(0, 1, self.curval % math.floor(self.curval))


class AcceleratedTransition(Transition):
    def __init__(self, acceleration: float = 1., accelerationmargin: float = 0.01, diffmargin: float = 0.0005,
                 maxvelocity: float = 100) -> None:
        super(AcceleratedTransition, self).__init__()
        self.acceleration: float = acceleration
        self.curval: float = 0.
        self.targetval: float = 0.
        self.velocity: float = 0.
        self.accelerationmargin: float = accelerationmargin
        self.diffmargin: float = diffmargin
        self.maxvelocity: float = maxvelocity

    def tick(self, dt: float) -> None:
        diff: float = self.targetval - self.curval
        if not diff == 0.0:  # Already there, no need to accelerate
            # Acceleration needed to come to a stop within distance to targetval
            stoppingacceleration: float = -(self.velocity ** 2) / (2 * diff) * (1. + self.accelerationmargin)

            # We can still accelerate towards the target
            if self.acceleration > abs(stoppingacceleration) or sign(self.velocity) != sign(diff):
                self.velocity += self.acceleration * dt * sign(diff)
            else:  # We need to stop!
                self.velocity -= self.acceleration * dt * sign(diff)
        if abs(self.velocity) > self.maxvelocity:
            self.velocity = sign(self.velocity) * self.maxvelocity

        # Prevent oscillations by setting curval to targetval if we're close enough or even overshooting slightly
        if abs(diff) < self.diffmargin:
            self.curval = self.targetval
            self.velocity = 0
        elif (sign(self.curval + self.velocity * dt) != sign(self.curval) and abs(
                self.velocity * dt) < self.diffmargin * 3):
            self.curval = self.targetval
            self.velocity = 0
        elif abs(self.velocity * dt) > abs(diff):  # Runaway oscillation protection
            self.curval = self.targetval
            self.velocity = 0
        else:
            self.curval += self.velocity * dt

    def hardset(self, value: float) -> None:
        self.curval = value
        self.targetval = value
        self.velocity = 0


class StepSequencer():
    def __init__(self, steps: List[Any], callback: FunctionType = None) -> None:
        self.steps: List[Any] = steps
        self.curindex: int = 0
        self.callback: FunctionType = callback

    def step(self) -> Any:
        self.curindex += 1
        if self.curindex >= len(self.steps):
            self.curindex = 0
        if self.callback is not None:
            self.callback(self.curindex, self.steps[self.curindex])
        return self.steps[self.curindex]

    def setsteps(self, steps: List[Any]) -> None:
        self.steps = steps
        if self.curindex >= len(self.steps):
            self.curindex = 0

    def setindex(self, index: int) -> Any:
        if 0 <= index < len(self.steps):
            self.curindex = index
        else:
            self.curindex = 0
        if self.callback is not None:
            self.callback(self.curindex, self.steps[self.curindex])
        return self.steps[self.curindex]


class PT1Transition(Transition):
    def __init__(self, timeconstant: float):
        super(PT1Transition, self).__init__()
        self.targetval = 0
        self.timeconstant: float = timeconstant

    def tick(self, dt):
        self.curval += (self.timeconstant * (self.targetval - self.curval)) * dt


class OnceTransition(Transition):
    def __init__(self):
        super(OnceTransition, self).__init__()

        self.trigd = False

    async def trig(self):
        self.settarget(1)
        await self.await_target_reached()

    def settarget(self, target: float) -> None:
        self.targetval = target
        self.trigd = True

    def __call__(self, *args, **kwargs) -> float:
        self.trigd = False
        return self.curval

    async def await_target_reached(self) -> None:
        while self.trigd:
            await asyncio.sleep(0)
        self.curval = 0
        self.targetval = 0
        return
