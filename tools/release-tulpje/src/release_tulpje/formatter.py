import logging


class RustishFormatter(logging.Formatter):
    COLOR_GREY = "\033[90m"
    COLOR_LIGHT_GREY = "\033[37m"
    COLOR_YELLOW = "\033[33m"
    COLOR_BLUE = "\033[34m"
    COLOR_GREEN = "\033[32m"
    COLOR_RED = "\033[31m"
    COLOR_BOLD_RED = "\033[31m"
    COLOR_CYAN = "\033[36m"
    RESET = "\033[0m"

    LEVEL_COLORS = {
        logging.DEBUG: COLOR_BLUE,
        logging.INFO: COLOR_GREEN,
        logging.WARNING: COLOR_YELLOW,
        logging.ERROR: COLOR_RED,
        logging.CRITICAL: COLOR_BOLD_RED,
    }
    LEVEL_NAME_MAP = {
        logging.DEBUG: "DEBUG",
        logging.INFO: "INFO",
        logging.WARNING: "WARN",
        logging.ERROR: "ERROR",
        logging.CRITICAL: "CRIT",
    }

    def __init__(self, *args, **kwargs):
        kwargs["fmt"] = (
            f"{self.COLOR_GREY}%(asctime)s.%(msecs)03dZ{self.RESET} %(levelname)14s {self.COLOR_GREY}%(name)s:{self.RESET} %(message)s"
        )
        kwargs["datefmt"] = "%Y-%m-%dT%H:%M:%S"
        super().__init__(*args, **kwargs)

    def format(self, record):
        record.levelname = (
            self.LEVEL_COLORS.get(record.levelno, "")
            + self.LEVEL_NAME_MAP.get(record.levelno, record.levelname)
            + self.RESET
        )
        return super().format(record)
