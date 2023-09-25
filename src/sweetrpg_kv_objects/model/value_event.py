# -*- coding: utf-8 -*-
__author__ = "Paul Schifferer <dm@sweetrpg.com>"
"""
"""

import logging
from sweetrpg_model_core.model.base import Model


class ValueEvent(Model):
    """A model object representing an event on a value."""

    def __init__(self, *args, **kwargs):
        """Creates a new ValueEvent object."""
        logging.debug("args: %s, kwargs: %s", args, kwargs)

        super().__init__(*args, **kwargs)

        self.store = kwargs.get("store")  # the store where the event occurred
        self.key = kwargs.get("key")  # the key whose value changed
        self.occurred = kwargs.get("occurred")  # the date/time the event occurred
