for standard_item_fragment_option_id in standard_item_fragment_option_ids:
    fetch_confirmed_standard_item_fragments("test", concierge_id)
    if (
        str(standard_item_fragment_option_id)
        in concierge_rule.form_rule["prefill_rules"]
    ):
        concierge_id = concierge_rule.form_rule["prefill_rules"][
            str(standard_item_fragment_option_id)
        ]

        fetch("test", concierge_id)

        if (
            uuid.UUID(concierge_id)
            in self.application.concierge_value["selected_ids"]
        ):
            return standard_item_fragment_option_id

    self.standard_item_repository.fetch_confirmed_standard_item_fragments("test", concierge_id)

self.standard_item_repository.fetch_confirmed_standard_item_fragments("test", concierge_id)
