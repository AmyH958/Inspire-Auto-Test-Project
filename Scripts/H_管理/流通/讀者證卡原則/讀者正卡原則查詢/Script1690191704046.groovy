import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('http://10.11.20.15/inspireapp/logon')

WebUI.maximizeWindow()

WebUI.waitForAlert(1)

WebUI.setText(findTestObject('Object Repository/管理/流通/讀者證卡原則/讀者證卡查詢/input_(15trunk)_member_id'), 'catc')

WebUI.setEncryptedText(findTestObject('Object Repository/管理/流通/讀者證卡原則/讀者證卡查詢/input_(15trunk)_member_pwd'), '9NLz+4tGZcQ=')

WebUI.setEncryptedText(findTestObject('Object Repository/管理/流通/讀者證卡原則/讀者證卡查詢/input_(15trunk)_member_pwd'), 'eh9+mX6n5G+m9qEgDyu51A==')

WebUI.enableSmartWait()

WebUI.click(findTestObject('Object Repository/管理/流通/讀者證卡原則/讀者證卡查詢/input_(15trunk)_Submit'))

WebUI.enableSmartWait()

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/讀者證卡原則/讀者證卡查詢/a_'))

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/讀者證卡原則/讀者證卡查詢/a__1'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/讀者證卡原則/讀者證卡查詢/a__1_2'), '讀者證卡原則')

WebUI.enhancedClick(findTestObject('Object Repository/管理/流通/讀者證卡原則/讀者證卡查詢/a__1_2'))

WebUI.verifyElementText(findTestObject('Object Repository/管理/流通/讀者證卡原則/讀者證卡查詢/span_'), '讀者身份類別:')

WebUI.selectOptionByValue(findTestObject('Object Repository/管理/流通/讀者證卡原則/讀者證卡查詢/select_010203040506070809101111112131415161_828bdf'), 
    '1', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/管理/流通/讀者證卡原則/讀者證卡查詢/select_010203040506070809101111112131415161_828bdf'), 
    '2', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/管理/流通/讀者證卡原則/讀者證卡查詢/select_010203040506070809101111112131415161_828bdf'), 
    '3', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/管理/流通/讀者證卡原則/讀者證卡查詢/select_010203040506070809101111112131415161_828bdf'), 
    '4', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/管理/流通/讀者證卡原則/讀者證卡查詢/select_010203040506070809101111112131415161_828bdf'), 
    '5', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/管理/流通/讀者證卡原則/讀者證卡查詢/select_010203040506070809101111112131415161_828bdf'), 
    '6', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/管理/流通/讀者證卡原則/讀者證卡查詢/select_010203040506070809101111112131415161_828bdf'), 
    '7', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/管理/流通/讀者證卡原則/讀者證卡查詢/select_010203040506070809101111112131415161_828bdf'), 
    '8', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/管理/流通/讀者證卡原則/讀者證卡查詢/select_010203040506070809101111112131415161_828bdf'), 
    '0', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/管理/流通/讀者證卡原則/讀者證卡查詢/select_()T-Pnew item 10L4020230418new item _5ed051'), 
    '1', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/管理/流通/讀者證卡原則/讀者證卡查詢/select_()T-Pnew item 10L4020230418new item _5ed051'), 
    '2', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/管理/流通/讀者證卡原則/讀者證卡查詢/select_()T-Pnew item 10L4020230418new item _5ed051'), 
    '3', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/管理/流通/讀者證卡原則/讀者證卡查詢/select_()T-Pnew item 10L4020230418new item _5ed051'), 
    '4', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/管理/流通/讀者證卡原則/讀者證卡查詢/select_()T-Pnew item 10L4020230418new item _5ed051'), 
    '5', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/管理/流通/讀者證卡原則/讀者證卡查詢/select_()T-Pnew item 10L4020230418new item _5ed051'), 
    '15', true)

WebUI.selectOptionByValue(findTestObject('Object Repository/管理/流通/讀者證卡原則/讀者證卡查詢/select_()T-Pnew item 10L4020230418new item _5ed051'), 
    '0', true)

WebUI.takeFullPageScreenshotAsCheckpoint('讀者證卡查詢')

WebUI.click(findTestObject('Object Repository/管理/流通/讀者證卡原則/讀者證卡查詢/a_PrefixNumber'))

WebUI.click(findTestObject('Object Repository/管理/流通/讀者證卡原則/讀者證卡查詢/a_PrefixNumber_1'))

WebUI.click(findTestObject('Object Repository/管理/流通/讀者證卡原則/讀者證卡查詢/a__1_2_3'))

WebUI.closeBrowser()

