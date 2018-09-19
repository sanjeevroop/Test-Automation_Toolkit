import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.checkpoint.CheckpointFactory as CheckpointFactory
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as MobileBuiltInKeywords
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testcase.TestCaseFactory as TestCaseFactory
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testdata.TestDataFactory as TestDataFactory
import com.kms.katalon.core.testobject.ObjectRepository as ObjectRepository
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WSBuiltInKeywords
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUiBuiltInKeywords
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys

WebUI.openBrowser('')

WebUI.navigateToUrl('http://mauw2k8sdworx:81/')

WebUI.setText(findTestObject('Temp/New Folder/19032018/20032018/input_Username'), 'Sanjeev@sdworx.com')

WebUI.setText(findTestObject('Temp/New Folder/19032018/20032018/input_Password'), '1234')

WebUI.click(findTestObject('Temp/New Folder/19032018/20032018/input_Submit'))

WebUI.click(findTestObject('Temp/New Folder/19032018/20032018/h4_YOUR SETUP'))

WebUI.click(findTestObject('Temp/New Folder/19032018/20032018/button_Your Setup'))

WebUI.click(findTestObject('Temp/New Folder/19032018/20032018/a_Your Pay  Benefits'))

WebUI.click(findTestObject('Temp/New Folder/19032018/20032018/a_MinimumRates'))

WebUI.maximizeWindow()

for (def row = 1; row <= findTestData('Minimumrates').getRowNumbers(); row++) {
WebUI.click(findTestObject('Temp/New Folder/19032018/20032018/a_Add'))

WebUI.setText(findTestObject('Temp/New Folder/19032018/20032018/input_date'), findTestData('Minimumrates').getValue(1, row))

WebUI.selectOptionByIndex(findTestObject('Temp/select_Please Select18 to 2021 (3)'), findTestData('Minimumrates').getValue(2, row))

WebUI.setText(findTestObject('Temp/New Folder/19032018/20032018/input_Value0'), findTestData('Minimumrates').getValue(3, 
            row))

WebUI.click(findTestObject('Temp/button_Save (4)'))
}

