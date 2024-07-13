# extractor

This crate handles extracting the used label keys from TypeScript files.
It removes the need to manually create labels.json files for namespaces.

Also a way for me to test purely building a library for the cli crate to consume.
Different ways of working I need to get used to.

## next-intl example

```typescript
// component.tsx
export const Component = () => {
    const t = useTranslations("Component");

    return (
        <div>
        <h1>{t("title")}</h1>
        <p>{t("paragraph")}</p>
        </div>
    )
}
```

Will output the following file:
```json
// Component.labels.json
{
    "title": "",
    "paragraph": ""
}
```


## Extra notes
- We could potentially get rid of *.labels.json files and purely focus on outputting this to a single JSON files with namespaces left intact
